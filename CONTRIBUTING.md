
# Contributing Guidelines

## Getting Started

1. Fork the repository  
2. Clone your fork  
3. Create your branch  
4. Make commits  
5. Push and create PR  
6. Profit!

## Code Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Always run `cargo fmt` before committing

## Code Quality

- Use [Clippy](https://doc.rust-lang.org/clippy/) for linting
- Run `cargo clippy` as part of your workflow

## PR naming

Name your commits whatever you like (within reason), but please follow the naming conventions for PRs.

Use something like this: "[Add] Cheese into pizza" or "[Fix] Lazy coders dont work"

| Type       | When use                                                                                | Go to changelog? |
|------------|-----------------------------------------------------------------------------------------|------------------|
| [Add]      | Was adding new functionality                                                            | ✅               |
| [Change]   | Was сhanged existing functionality                                                      | ✅               |
| [Fix ]     | Was fixed bug                                                                           | ✅               |
| [Remove]   | Was removed functionality                                                               | ✅               |
| [Refactor] | Was changed functionality without changed behavior                                      | ❌               |
| [Docs]     | Was changed docs                                                                        | ❌               |
| [Test]     | Was changed test                                                                        | ❌               |
| [Chore]    | Was changed something internal that doesn't affect the user for example, Update README. | ❌               |

## AI usage

1) Ownership. As a contributor, you are responsible for everything you submit. You must personally understand every line of code or documentation that you put forward for review, and be able to articulate the design rationale and implementation tradeoffs when asked. Work within your skill level and knowledge of the project.
2) No AI-authored commits. Only humans can be commit authors
 