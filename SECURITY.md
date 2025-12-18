# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it by:

1. **DO NOT** create a public GitHub issue
2. Email the maintainers directly (or use GitHub's private vulnerability reporting)
3. Include detailed steps to reproduce the issue
4. Allow reasonable time for a fix before public disclosure

## Security Considerations

### Educational Tool Disclaimer

Entropy Forge is primarily an **educational tool** for understanding cryptographic concepts. 

**DO NOT use this software for:**
- Production cryptographic applications
- Protecting sensitive data
- Any security-critical systems

### Cryptographic Implementations

The cryptographic implementations in this project (e.g., XOR stream cipher) are simplified for educational purposes and are **NOT cryptographically secure** for real-world use.

For production cryptography, use well-audited libraries such as:
- [RustCrypto](https://github.com/RustCrypto)
- [ring](https://github.com/briansmith/ring)
- [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide)

## Dependencies

We use Dependabot to automatically monitor and update dependencies for security vulnerabilities.

## Acknowledgments

We appreciate responsible disclosure of security issues.
