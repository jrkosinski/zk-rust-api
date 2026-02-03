# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of zk-rust-api seriously. If you discover a security vulnerability, please follow these steps:

### Please Do Not

- Do not open a public GitHub issue for security vulnerabilities
- Do not disclose the vulnerability publicly until it has been addressed

### Please Do

1. **Email us directly** at security@example.com with:
   - A description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Any suggested fixes (if applicable)

2. **Allow time for response**: We will acknowledge your email within 48 hours and provide a detailed response within 7 days.

3. **Coordinate disclosure**: We will work with you to understand and address the issue before any public disclosure.

## Security Best Practices

When using zk-rust-api:

1. **Keep dependencies updated**: Regularly update to the latest version
2. **Use cargo audit**: Run `cargo audit` to check for known vulnerabilities
3. **Review dependencies**: Be aware of what dependencies you're including
4. **Validate inputs**: Always validate and sanitize user inputs
5. **Follow Rust security guidelines**: Adhere to Rust security best practices

## Security Update Process

1. Security issues are given high priority
2. Patches are developed and tested privately
3. A security advisory is published with the fix
4. Users are notified through GitHub Security Advisories
5. A new version is released with the fix

## Known Security Considerations

### Cryptographic Implementation

This library uses Halo2 for zero-knowledge proofs. Users should:

- Understand the cryptographic assumptions
- Use appropriate parameters for their security requirements
- Stay informed about updates to the underlying cryptographic libraries

### Side-Channel Attacks

- Be aware of potential timing attacks
- Consider constant-time operations for sensitive data
- Review security implications of your specific use case

## Recognition

We appreciate the security research community and will acknowledge researchers who responsibly disclose vulnerabilities (unless they prefer to remain anonymous).

## Questions?

For questions about security that don't involve reporting a vulnerability, please open a discussion on GitHub.

---

Last updated: 2026-02-03
