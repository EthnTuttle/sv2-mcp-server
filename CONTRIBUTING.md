# Contributing to Stratum V2 MCP Server

Thank you for your interest in contributing to the Stratum V2 MCP Server! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

We welcome contributions from the community! Here are the main ways you can contribute:

- **🐛 Bug Reports**: Report bugs and issues
- **✨ Feature Requests**: Suggest new features
- **📝 Documentation**: Improve documentation
- **🔧 Code Contributions**: Submit code changes
- **🧪 Testing**: Help with testing and test coverage
- **💡 Ideas**: Share ideas and feedback

## 🚀 Getting Started

### Prerequisites

- **Rust**: 1.70+ ([Install Rust](https://rustup.rs/))
- **Git**: Latest version
- **GitHub Account**: For submitting issues and pull requests

### Development Setup

1. **Fork the repository**:
   ```bash
   # Fork on GitHub first, then clone your fork
   git clone https://github.com/average-gary/sv2-mcp-server.git
   cd sv2-mcp-server
   ```

2. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/original-username/sv2-mcp-server.git
   ```

3. **Install dependencies**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

5. **Run linter**:
   ```bash
   cargo clippy
   ```

6. **Format code**:
   ```bash
   cargo fmt
   ```

## 📋 Development Workflow

### 1. Create a Feature Branch

Always work on a feature branch, never directly on `main`:

```bash
# Create and switch to a new feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/your-bug-description
```

### 2. Make Your Changes

- Write clear, well-documented code
- Follow Rust conventions and best practices
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific tests
cargo test test_name

# Run linter
cargo clippy

# Check formatting
cargo fmt --check
```

### 4. Commit Your Changes

Use conventional commit messages:

```bash
# Format: type(scope): description
git commit -m "feat(tlv): add support for custom TLV field types"
git commit -m "fix(protocol): resolve message encoding issue"
git commit -m "docs(readme): update installation instructions"
```

**Commit Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### 5. Push and Create Pull Request

```bash
# Push your branch
git push origin feature/your-feature-name

# Create a Pull Request on GitHub
```

## 📝 Pull Request Guidelines

### Before Submitting

- [ ] Code follows Rust conventions
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No linter warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages are clear and descriptive

### Pull Request Template

When creating a Pull Request, please include:

1. **Description**: What does this PR do?
2. **Type of Change**: Bug fix, feature, documentation, etc.
3. **Testing**: How was this tested?
4. **Breaking Changes**: Any breaking changes?
5. **Related Issues**: Links to related issues

### Example Pull Request

```markdown
## Description
Adds support for custom TLV field types in the Stratum V2 protocol.

## Type of Change
- [x] New feature (non-breaking change which adds functionality)
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)

## Testing
- [x] Added unit tests for new TLV field functionality
- [x] Ran existing test suite to ensure no regressions
- [x] Tested with real Stratum V2 mining pool

## Breaking Changes
None - this is a backward-compatible addition.

## Related Issues
Closes #123
```

## 🧪 Testing Guidelines

### Writing Tests

- Write tests for all new functionality
- Use descriptive test names
- Test both success and failure cases
- Mock external dependencies when appropriate

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tlv_field_creation() {
        // Arrange
        let extension_type = 2;
        let field_type = 1;
        let value = "worker123";

        // Act
        let result = create_tlv_field(extension_type, field_type, value);

        // Assert
        assert!(result.is_ok());
        let tlv = result.unwrap();
        assert_eq!(tlv.extension_type, extension_type);
        assert_eq!(tlv.field_type, field_type);
    }

    #[test]
    fn test_tlv_field_validation_failure() {
        // Test invalid input
        let result = create_tlv_field(0, 0, "");
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin

# Run tests in specific module
cargo test tests::test_module

# Run tests with output
cargo test -- --nocapture
```

## 📚 Documentation Guidelines

### Code Documentation

- Document all public functions and types
- Use Rust doc comments (`///`)
- Include examples in documentation
- Keep documentation up to date

```rust
/// Creates a TLV (Type-Length-Value) field for Stratum V2 protocol.
///
/// # Arguments
///
/// * `extension_type` - The extension type (16-bit)
/// * `field_type` - The field type within the extension (8-bit)
/// * `value` - The field value as a string
///
/// # Returns
///
/// Returns a `Result<TlvField, TlvError>` containing the created TLV field
/// or an error if creation fails.
///
/// # Examples
///
/// ```
/// use stratum_v2_mcp_server::create_tlv_field;
///
/// let tlv = create_tlv_field(2, 1, "worker123")?;
/// assert_eq!(tlv.extension_type, 2);
/// ```
pub fn create_tlv_field(
    extension_type: u16,
    field_type: u8,
    value: &str,
) -> Result<TlvField, TlvError> {
    // Implementation...
}
```

### README Updates

- Update README.md for new features
- Include usage examples
- Update installation instructions if needed
- Keep the table of contents current

## 🐛 Bug Reports

### Before Reporting

- Check existing issues to avoid duplicates
- Try to reproduce the issue
- Gather relevant information

### Bug Report Template

```markdown
## Bug Description
Brief description of the bug.

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: [e.g., macOS 14.0]
- Rust Version: [e.g., 1.75.0]
- Crate Version: [e.g., 0.1.0]

## Additional Information
Any other relevant information, logs, screenshots, etc.
```

## ✨ Feature Requests

### Before Requesting

- Check if the feature already exists
- Consider if it aligns with project goals
- Think about implementation complexity

### Feature Request Template

```markdown
## Feature Description
Brief description of the requested feature.

## Use Case
Why is this feature needed? What problem does it solve?

## Proposed Solution
How should this feature work?

## Alternatives Considered
What other approaches were considered?

## Additional Information
Any other relevant information.
```

## 🤝 Code Review Process

### Review Guidelines

- Be respectful and constructive
- Focus on the code, not the person
- Provide specific, actionable feedback
- Suggest improvements when possible

### Responding to Reviews

- Address all review comments
- Make requested changes
- Ask questions if something is unclear
- Thank reviewers for their time

## 📋 Issue Labels

We use the following labels to categorize issues:

- `bug`: Something isn't working
- `enhancement`: New feature or request
- `documentation`: Improvements or additions to documentation
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `question`: Further information is requested
- `wontfix`: This will not be worked on

## 🏷️ Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] All tests pass
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version is bumped in Cargo.toml
- [ ] Release notes are written
- [ ] Tag is created on GitHub

## 📞 Getting Help

If you need help with contributing:

- **Issues**: [Open an issue](https://github.com/average-gary/sv2-mcp-server/issues)
- **Discussions**: [Start a discussion](https://github.com/average-gary/sv2-mcp-server/discussions)
- **Documentation**: Check this file and the README

## 🙏 Recognition

Contributors will be recognized in:

- The project README
- Release notes
- GitHub contributors page

Thank you for contributing to the Stratum V2 MCP Server! 🚀 