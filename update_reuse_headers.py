#!/usr/bin/env python3

import os
import sys
import re
import subprocess
from datetime import datetime, timezone
from collections import defaultdict
import concurrent.futures
import threading
import time

# --- Configuration ---
CUTOFF_COMMIT_HASH = "a883d9cf5605cae85c5dc8b62722b29694c79ce0"
LICENSE_BEFORE = "WTFPL"
LICENSE_AFTER = "AGPL-3.0-or-later"
REPO_PATH = "."
MAX_WORKERS = os.cpu_count() or 4

FILE_PATTERNS = [
    "*.rs", "*.toml", "*.md", "*.sh", 
    "*.yml", "*.yaml", "*.json"
]

COMMENT_STYLES = {
    ".rs": ("//", None),
    ".toml": ("#", None),
    ".md": ("<!--", "-->"),
    ".sh": ("#", None),
    ".yml": ("#", None),
    ".yaml": ("#", None),
    ".json": ("//", None),
    ".lock": ("//", None)
}

# --- Shared State ---
progress_lock = threading.Lock()
processed_count = 0
skipped_count = 0
error_count = 0
wtfpl_count = 0
agpl_count = 0
last_file_processed = ""
last_license_type = ""
all_warnings = []
total_files = 0

def run_git_command(command, cwd=REPO_PATH, check=True):
    """Run git command and return output."""
    try:
        result = subprocess.run(
            command,
            capture_output=True,
            text=True,
            check=check,
            cwd=cwd,
            encoding='utf-8',
            errors='ignore'
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        if check:
            print(f"Error running git command {' '.join(command)}: {e.stderr}", 
                  file=sys.stderr)
        return None

def get_git_authors(file_path):
    """Get authors and their contribution years for a file."""
    cmd = ["git", "log", "--follow", "--pretty=format:%an <%ae>", "--", file_path]
    output = run_git_command(cmd, check=False)
    if not output:
        return {}
    
    authors = defaultdict(list)
    for line in output.splitlines():
        if line.strip():
            authors[line.strip()].append(1)
    
    return {author: (0, 0) for author in authors}

def get_commit_timestamp(commit_hash):
    """Get timestamp for a specific commit."""
    cmd = ["git", "show", "-s", "--format=%ct", commit_hash]
    output = run_git_command(cmd)
    return int(output) if output else None

def get_file_last_commit_timestamp(file_path):
    """Get timestamp of last commit that modified the file."""
    cmd = ["git", "log", "-1", "--format=%ct", "--follow", "--", file_path]
    output = run_git_command(cmd, check=False)
    return int(output) if output else None

def parse_existing_header(content, comment_style):
    """Parse existing REUSE header."""
    prefix, suffix = comment_style
    lines = content.splitlines()
    authors = {}
    license_id = None
    header_lines = []
    
    if suffix is None:
        for line in lines:
            if line.startswith(prefix + " SPDX-FileCopyrightText:"):
                parts = line[len(prefix):].split(":", 1)
                if len(parts) > 1:
                    copyright_info = parts[1].strip()
                    year_match = re.match(r"(\d{4})", copyright_info)
                    if year_match:
                        year = int(year_match.group(1))
                        author = copyright_info[year_match.end():].strip()
                        authors[author] = (year, year)
                header_lines.append(line)
            elif line.startswith(prefix + " SPDX-License-Identifier:"):
                license_id = line.split(":", 1)[1].strip()
                header_lines.append(line)
            elif line.strip() == prefix:
                header_lines.append(line)
            elif not line.strip():
                continue
            else:
                break
    
    return authors, license_id, header_lines

def create_header(authors, license_id, comment_style):
    """Create REUSE header."""
    prefix, suffix = comment_style
    lines = []
    
    for author in sorted(authors.keys()):
        year = authors[author][1]
        lines.append(f"{prefix} SPDX-FileCopyrightText: {year} {author}")
    
    lines.append(f"{prefix}")
    lines.append(f"{prefix} SPDX-License-Identifier: {license_id}")
    lines.append("")
    
    return "\n".join(lines)

def process_file(file_path):
    """Process a single file."""
    global processed_count, skipped_count, error_count
    global wtfpl_count, agpl_count, last_file_processed, last_license_type
    
    file_ext = os.path.splitext(file_path)[1].lower()
    comment_style = COMMENT_STYLES.get(file_ext)
    
    if not comment_style:
        with progress_lock:
            skipped_count += 1
        return "skipped_unsupported"
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        if "SPDX-License-Identifier" in content[:1000]:
            with progress_lock:
                skipped_count += 1
            return "skipped_has_header"
        
        # Get authors (fallback to current user if git fails)
        try:
            authors = get_git_authors(file_path)
        except Exception as e:
            print(f"Warning: Could not get authors for {file_path}: {str(e)}")
            authors = {}
            try:
                name = run_git_command(["git", "config", "user.name"])
                email = run_git_command(["git", "config", "user.email"])
                if name and email:
                    current_year = datetime.now().year
                    authors[f"{name} <{email}>"] = (current_year, current_year)
            except:
                pass
        
        # Get license based on commit dates
        file_ts = get_file_last_commit_timestamp(file_path)
        cutoff_ts = get_commit_timestamp(CUTOFF_COMMIT_HASH)
        
        if file_ts is not None and cutoff_ts is not None:
            license_id = LICENSE_AFTER if file_ts > cutoff_ts else LICENSE_BEFORE
        else:
            license_id = LICENSE_AFTER  # Fallback
        
        header = create_header(authors, license_id, comment_style)
        new_content = header + "\n" + content
        
        if new_content != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            
            with progress_lock:
                processed_count += 1
                last_file_processed = file_path
                last_license_type = license_id
                if license_id == LICENSE_BEFORE:
                    wtfpl_count += 1
                else:
                    agpl_count += 1
            
            return "updated"
        
        with progress_lock:
            skipped_count += 1
        return "skipped_no_change"
    
    except Exception as e:
        with progress_lock:
            error_count += 1
            all_warnings.append(f"Error processing {file_path}: {str(e)}")
        return "error"

def print_progress():
    """Print processing progress."""
    with progress_lock:
        total = processed_count + skipped_count + error_count
        percent = (total / total_files) * 100 if total_files > 0 else 0
        progress = f"[{'#' * int(percent//2)}{' ' * (50 - int(percent//2))}] {percent:.1f}%"
        status = (
            f"Processed: {total}/{total_files} | "
            f"WTFPL: {wtfpl_count} | AGPL: {agpl_count} | "
            f"Last: {os.path.basename(last_file_processed) if last_file_processed else 'N/A'}"
        )
        sys.stdout.write(f"\r{progress} {status}")
        sys.stdout.flush()

def main():
    global total_files
    
    print("REUSE Header Updater for Rust Projects")
    print("=" * 50)
    
    # Verify cutoff commit exists
    cutoff_ts = get_commit_timestamp(CUTOFF_COMMIT_HASH)
    if cutoff_ts is None:
        print(f"Error: Could not find cutoff commit {CUTOFF_COMMIT_HASH}", file=sys.stderr)
        return 1
    
    print(f"Cutoff commit timestamp: {cutoff_ts}")
    
    # Get list of files
    git_cmd = ["git", "ls-files"] + FILE_PATTERNS
    files_output = run_git_command(git_cmd)
    
    if not files_output:
        print("Error: No files found or git command failed", file=sys.stderr)
        return 1
    
    target_files = [f for f in files_output.splitlines() if f.strip()]
    total_files = len(target_files)
    
    if not target_files:
        print("No files found to process")
        return 0
    
    print(f"Found {total_files} files to process")
    print("Starting processing...")
    
    # Process files in parallel
    with concurrent.futures.ThreadPoolExecutor(max_workers=MAX_WORKERS) as executor:
        futures = [executor.submit(process_file, f) for f in target_files]
        
        while True:
            done = sum(f.done() for f in futures)
            print_progress()
            if done == len(futures):
                break
            time.sleep(0.1)
    
    # Print summary
    print("\n\nProcessing complete!")
    print("=" * 50)
    print(f"Total files: {total_files}")
    print(f"Updated: {processed_count}")
    print(f"Skipped: {skipped_count}")
    print(f"Errors: {error_count}")
    print(f"WTFPL licenses: {wtfpl_count}")
    print(f"AGPL licenses: {agpl_count}")
    
    if all_warnings:
        print("\nWarnings/Errors:")
        for warning in all_warnings[:10]:
            print(f" - {warning}")
        if len(all_warnings) > 10:
            print(f" - ... and {len(all_warnings)-10} more")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())