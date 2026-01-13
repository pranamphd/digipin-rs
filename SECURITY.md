## Reporting a vulnerability

If you've discovered a security issue, please do not open a public issue. Instead:

- Use GitHub's private security advisory feature, OR
- Email the project security team at: digipin-security@pranam.phd

Please include (in `plaintext` and English):

- A description of the issue
- Step-by-step reproduction
- Impact and suggested remediation

## Supported Versions

| Version       | Supported          |
| ------------- | ------------------ |
| 0.1.x         | :white_check_mark: |
| 0.1.x-beta.x         | :white_check_mark: |
| 0.1.x-alpha.x | :x:                |

## Supply Chain Security (SLSA)

This project uses GitHub Actions with OIDC-based provenance attestation.

### SLSA Level

- **SLSA Build Level 3 (GitHub-hosted)** for GitHub release artifacts

### What is covered

- `.crate` artifact produced via `cargo package`
- SBOMs (CycloneDX + SPDX)
- SHA256 checksums

### What is not covered

- crates.io registry ingestion (crates.io does not currently verify SLSA attestations)

### Verification

Users may verify provenance using:

```bash
slsa-verifier verify-artifact digipin-rs-<version>.crate \
  --source-uri github.com/pranamphd/digipin-rs \
  --source-tag v<version>
```
