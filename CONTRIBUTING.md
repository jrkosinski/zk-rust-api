# Contributing to zk-rust-api

Thank you for your interest in contributing to zk-rust-api! We welcome contributions from the community.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Coding Standards](#coding-standards)
- [Submitting Changes](#submitting-changes)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](./CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment
4. Create a new branch for your changes
5. Make your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A text editor or IDE with Rust support

### Setup Instructions

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/zk-rust-api.git
cd zk-rust-api

# Add upstream remote
git remote add upstream https://github.com/yourusername/zk-rust-api.git

# Install dependencies
cargo build

# Run tests to verify setup
cargo test
```

## Making Changes

### Branching Strategy

- Create a new branch from `main` for each feature or bugfix
- Use descriptive branch names:
  - `feature/add-new-circuit`
  - `fix/proof-verification-bug`
  - `docs/update-readme`

```bash
git checkout -b feature/your-feature-name
```

### Commit Messages

Write clear, concise commit messages following these guidelines:

- Use the imperative mood ("Add feature" not "Added feature")
- Limit the first line to 72 characters
- Reference issues and pull requests where appropriate
- Consider using conventional commits format:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `test:` for test additions/changes
  - `refactor:` for code refactoring
  - `perf:` for performance improvements
  - `chore:` for maintenance tasks

Example:
```
feat: add support for custom circuit parameters

- Allow users to specify custom parameters for circuits
- Add validation for parameter ranges
- Update documentation with usage examples

Closes #123
```

## Testing

All code changes should include appropriate tests.

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (if using cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Writing Tests

- Write unit tests for individual functions
- Write integration tests for public API
- Add benchmarks for performance-critical code
- Ensure tests are deterministic and don't rely on external state

## Coding Standards

### Code Style

We use `rustfmt` and `clippy` to maintain code quality:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Fix clippy suggestions
cargo clippy --fix
```

### Documentation

- Add doc comments to all public APIs
- Include examples in doc comments
- Update README.md if adding new features
- Add inline comments for complex logic

```rust
/// Generates a zero-knowledge proof for the given circuit.
///
/// # Arguments
///
/// * `circuit` - The circuit to generate a proof for
/// * `params` - Parameters for proof generation
///
/// # Examples
///
/// ```
/// use zk_rust_api::generate_proof;
///
/// let proof = generate_proof(circuit, params)?;
/// ```
///
/// # Errors
///
/// Returns an error if proof generation fails.
pub fn generate_proof(circuit: Circuit, params: Params) -> Result<Proof> {
    // Implementation
}
```

### Security Considerations

- Never commit sensitive information (keys, passwords, etc.)
- Be cautious with cryptographic implementations
- Follow Rust security best practices
- Use `cargo audit` to check for known vulnerabilities

```bash
cargo audit
```

## Submitting Changes

### Pull Request Process

1. **Update your branch** with the latest changes from upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Ensure all tests pass**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

3. **Push your changes**:
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create a Pull Request**:
   - Go to the repository on GitHub
   - Click "New Pull Request"
   - Select your branch
   - Fill out the PR template
   - Link any related issues

### Pull Request Guidelines

- Provide a clear description of the changes
- Include motivation and context
- List any breaking changes
- Add screenshots for UI changes
- Ensure CI passes
- Request review from maintainers
- Be responsive to feedback
- Keep PRs focused and atomic

### PR Template

Your PR should include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] New tests added
- [ ] Benchmarks run (if applicable)

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings generated
```

## Reporting Bugs

### Before Submitting a Bug Report

- Check existing issues to avoid duplicates
- Verify the bug in the latest version
- Collect relevant information

### Bug Report Template

```markdown
**Describe the bug**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. ...
2. ...

**Expected behavior**
What you expected to happen

**Actual behavior**
What actually happened

**Environment:**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.70.0]
- Project version: [e.g., 0.1.0]

**Additional context**
Any other relevant information
```

## Feature Requests

We welcome feature requests! Please:

1. Check if the feature already exists
2. Check if there's an open issue for it
3. Provide a clear use case
4. Explain why this feature would be useful
5. Consider submitting a PR to implement it

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
Clear description of the problem

**Describe the solution you'd like**
Clear description of desired solution

**Describe alternatives you've considered**
Alternative solutions or features

**Additional context**
Any other relevant information
```

## Questions?

If you have questions:
- Check the [documentation](https://docs.rs/zk-rust-api)
- Open a discussion on GitHub
- Reach out to maintainers

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- GitHub contributors page

Thank you for contributing to zk-rust-api!
