/// Data structure for storing a [SparseSet].
pub struct SparseSet<T> {
    dense: Vec<Entry<T>>,
    sparse: Vec<usize>,
}

/// Represents an entry in the [SparseSet].
struct Entry<T> {
    key: u64,
    value: T,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet {
            dense: Vec::new(),
            sparse: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: u64, value: T) {
        if key >= self.sparse.len() as u64 {
            self.sparse.resize((key + 1) as usize, usize::MAX);
        }

        if self.sparse[key as usize] != usize::MAX {
            let idx = self.sparse[key as usize];
            self.dense[idx].value = value;
        } else {
            self.sparse[key as usize] = self.dense.len();
            self.dense.push(Entry { key, value });
        }
    }

    pub fn get(&self, key: u64) -> Option<&T> {
        if (key as usize) < self.sparse.len() {
            let idx = self.sparse[key as usize];
            if idx != usize::MAX {
                return Some(&self.dense[idx].value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: u64) -> Option<&mut T> {
        if (key as usize) < self.sparse.len() {
            let idx = self.sparse[key as usize];
            if idx != usize::MAX {
                return Some(&mut self.dense[idx].value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: u64) -> Option<T> {
        if (key as usize) >= self.sparse.len() {
            return None;
        }

        let dense_idx = self.sparse[key as usize];
        if dense_idx == usize::MAX {
            return None;
        }

        let removed_entry = self.dense.swap_remove(dense_idx);
        self.sparse[key as usize] = usize::MAX;

        if dense_idx < self.dense.len() {
            let swapped_key = self.dense[dense_idx].key;
            self.sparse[swapped_key as usize] = dense_idx;
        }

        Some(removed_entry.value)
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }
}
