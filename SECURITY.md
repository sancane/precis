# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Please DO NOT report security vulnerabilities through public GitHub issues.**

We take security seriously and appreciate responsible disclosure. If you discover a security vulnerability in this project, please report it privately.

### How to Report

**Preferred Method - GitHub Security Advisory:**
1. Go to https://github.com/sancane/precis/security/advisories
2. Click "Report a vulnerability"
3. Fill in the details using the template below

**Alternative - Email:**
sancane@gmail.com (please use subject: "[SECURITY] PRECIS Vulnerability Report")

### What to Include

Please include the following information in your report:

- **Description**: Brief description of the vulnerability
- **Type**: Type of vulnerability (e.g., buffer overflow, injection, authentication bypass)
- **Impact**: What an attacker could achieve by exploiting this
- **Affected Components**: Which crates/modules are affected
  - `precis-core`
  - `precis-profiles`
  - `precis-tools`
- **Affected Versions**: Which versions are vulnerable
- **Location**: Full paths of affected source files
- **Source**: Tag/branch/commit or direct link to affected code
- **Reproduction**: Step-by-step instructions to reproduce the issue
- **Proof of Concept**: Code that demonstrates the vulnerability (if available)
- **Suggested Fix**: Your ideas for fixing the issue (if any)

### Example Report

```markdown
**Description**: Integer overflow in string length calculation

**Type**: Integer Overflow

**Impact**: Could lead to buffer overflows when processing extremely long strings

**Affected**: precis-core v0.1.0 - v0.1.11

**Location**: precis-core/src/stringclasses.rs:123

**Reproduction**:
1. Create a string with length > u32::MAX
2. Call IdentifierClass::allows()
3. Observe panic/overflow

**PoC**: [attach code or link to gist]

**Suggested Fix**: Use checked arithmetic and return proper error
```

## Response Timeline

We are committed to responding quickly to security reports:

- **Initial Response**: Within 48 hours of receiving your report
- **Status Update**: Within 7 days with assessment of the issue
- **Fix Timeline**: Depends on severity (see below)

### Severity Levels

| Severity | Response Time | Example |
|----------|---------------|---------|
| **Critical** | 24-48 hours | Remote code execution, authentication bypass |
| **High** | 7 days | Memory corruption, significant data exposure |
| **Medium** | 30 days | Denial of service, information disclosure |
| **Low** | 90 days | Minor information leak, configuration issues |

## Disclosure Policy

We follow **coordinated disclosure**:

1. **Confirmation**: We confirm receipt and validate the vulnerability
2. **Assessment**: We assess severity and impact
3. **Fix Development**: We develop and test a fix
4. **Coordinated Release**: We coordinate disclosure timing with reporter
5. **Public Disclosure**: We publish:
   - Security advisory on GitHub
   - Updated versions on crates.io
   - Entry in CHANGELOG.md
   - CVE if applicable

**Typical Timeline**: 90 days from report to public disclosure (adjustable based on complexity)

## Security Considerations for Users

### Using This Library Safely

This library processes internationalized strings and implements PRECIS profiles (RFC 8264, 8265, 8266). While we strive for security, please be aware:

1. **Input Validation**: Always validate input strings before passing to PRECIS functions
   ```rust
   // Good: Validate length first
   if input.len() > MAX_USERNAME_LENGTH {
       return Err(ValidationError::TooLong);
   }
   let username = UsernameCaseMapped::enforce(input)?;
   ```

2. **Resource Limits**: Be cautious with untrusted input that could consume resources
   - Very long strings (> 1MB)
   - Strings with complex Unicode sequences
   - High-volume processing scenarios

3. **Error Handling**: Always handle errors properly
   ```rust
   match Nickname::enforce(user_input) {
       Ok(nickname) => process_nickname(nickname),
       Err(e) => {
           // Log error but don't expose internal details to users
           log::error!("Nickname validation failed: {:?}", e);
           return Err(UserError::InvalidNickname);
       }
   }
   ```

4. **Timing Attacks**: The `compare()` functions perform string comparison after normalization. While not constant-time, this is acceptable for the PRECIS use case (usernames, nicknames). For password comparison, use dedicated password hashing libraries (argon2, bcrypt).

### Known Limitations

See [Known Limitations](README.md#known-limitations) in README.md for documented limitations including:
- Case mapping behavior (unconditional, no tailoring)
- Unicode version differences between core and profiles

## Security Features

### What We Do

- ✅ **100% Safe Rust**: No `unsafe` code blocks
- ✅ **Comprehensive Testing**: Extensive test suite including property-based and fuzzing
- ✅ **Continuous Fuzzing**: ClusterFuzzLite runs on every PR
- ✅ **Dependency Audits**: Daily security audits via cargo-audit
- ✅ **Minimal Dependencies**: Very few runtime dependencies
- ✅ **License Compliance**: Verified via cargo-deny
- ✅ **MSRV Validation**: CI ensures compilation with declared MSRV

### Dependency Security

We use:
- `cargo-audit` (daily checks for known vulnerabilities)
- `cargo-deny` (license and source verification)
- `dependabot` (automated dependency updates - coming soon)

View our dependency security status:
[![dependency status](https://deps.rs/repo/github/sancane/precis/status.svg)](https://deps.rs/repo/github/sancane/precis)

## Hall of Fame

We recognize and thank security researchers who responsibly disclose vulnerabilities:

<!-- Add researchers here as issues are reported -->

*No vulnerabilities reported yet.*

---

## Questions?

If you have questions about:
- **This security policy**: Open a discussion on GitHub
- **General usage**: Check [CONTRIBUTING.md](CONTRIBUTING.md) or open an issue
- **Security vulnerability**: Use GitHub Security Advisory or email sancane@gmail.com

Thank you for helping keep PRECIS and its users safe! 🔒
