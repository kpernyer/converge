# Security Policy

## Supported Versions

We actively support security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:

1. **Email**: Send details to `security@aprio.one`
2. **GitHub Security Advisory**: Use the [GitHub Security Advisory](https://github.com/your-org/converge/security/advisories/new) feature (if available)

### What to Include

When reporting a vulnerability, please include:

- **Type of vulnerability** (e.g., buffer overflow, SQL injection, XSS)
- **Location of the affected code** (file paths, line numbers, or commit hashes)
- **Step-by-step instructions** to reproduce the issue
- **Potential impact** of the vulnerability
- **Suggested fix** (if you have one)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Resolution**: Depends on severity and complexity

### Security Best Practices

When working with Converge:

1. **Never commit API keys or secrets** — Use environment variables or secret managers
2. **Keep dependencies updated** — Run `cargo audit` regularly
3. **Validate all inputs** — Especially from LLM providers and external APIs
4. **Use HTTPS** — For all external communications
5. **Follow principle of least privilege** — Agents should only access what they need

### Known Security Considerations

#### LLM Provider Integration

- **API Keys**: Store in secret managers (Google Secret Manager, Vault), never in code
- **Rate Limiting**: Implement rate limiting for LLM API calls
- **Input Validation**: Validate all LLM responses before promoting to Facts
- **Provenance Tracking**: All LLM outputs include provenance for auditability

#### Context and Data

- **Context Isolation**: Each Root Intent has isolated context
- **No Cross-Intent Data Leakage**: Contexts are never shared between intents
- **Append-Only**: Context is monotonic, preventing data tampering

#### Agent Execution

- **Read-Only Agents**: Agents cannot mutate context directly
- **Deterministic Execution**: Same inputs produce same outputs
- **No Hidden State**: All state is explicit in context

### Security Disclosure Process

1. **Report** the vulnerability privately
2. **Acknowledge** receipt within 48 hours
3. **Investigate** and confirm the vulnerability
4. **Develop** a fix in a private branch
5. **Test** the fix thoroughly
6. **Release** the fix with appropriate version bump
7. **Disclose** publicly after users have had time to update

### Credits

We maintain a [SECURITY.md](SECURITY.md) file and will credit security researchers who responsibly disclose vulnerabilities (with their permission).

---

**Thank you for helping keep Converge and its users safe!**

