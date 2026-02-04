# Contributing to TITAN-CLI

Thank you for your interest in contributing to TITAN-CLI! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful and inclusive. We welcome contributions from everyone.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/TITAN-CLI-L5-Terminal-Commander.git`
3. Create a feature branch: `git checkout -b feat/your-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Commit with conventional format: `git commit -m 'feat: add new feature'`
7. Push to your fork: `git push origin feat/your-feature`
8. Open a Pull Request

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks
- `perf:` Performance improvements

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` to check for lints
- Follow Rust naming conventions
- Add documentation for public items

## Testing

- Write unit tests for new functionality
- Ensure all tests pass: `cargo test`
- Add integration tests for commands

## Pull Request Process

1. Update documentation if needed
2. Add changelog entry
3. Ensure CI passes
4. Request review from maintainers
5. Address review comments

## Questions?

Open an issue for any questions or discussions.
