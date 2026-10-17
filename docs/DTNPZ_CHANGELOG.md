# dtnpz edition changelog

## 0.1.4-dtnpz.1 — 2026-10-17

- Forked from Duck ToolBox upstream v0.1.4-stable.
- Added opt-in read-only joyeuse/miatoll diagnostic script for Android properties, library and device-node availability and the original backend's prerequisite status.
- Isolated fork update metadata, version numbering and GitHub release links.
- Preserved upstream RKP behavior, module ID, and MIT attribution.
- Added automated fork metadata and shell parsing smoke tests.
- Changed Device RKP Issuer contract from 5 to 6: JSON reports paths instead of embedding keybox XML and its private key.
- Hardened Device RKP Issuer certificate-path validation to use cryptographic signatures, CA/key-usage and validity checks before keybox export; added tampering regression tests.

The upstream implementation still reports real-firmware compatibility and Google certificate issuance as unverified for relevant new backends.
