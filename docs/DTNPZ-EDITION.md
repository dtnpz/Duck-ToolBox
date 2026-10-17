# dtnpz edition of Duck ToolBox

This is a **personal fork** of [eltavine/Duck-ToolBox](https://github.com/eltavine/Duck-ToolBox). It preserves the upstream MIT license and attribution.

## What changed

- Adds `module/bin/joyeuse-diag.sh`, a **read-only** hardware and firmware prerequisites report for Qualcomm joyeuse/miatoll devices.
- Uses the `dtnpz` GitHub release and update URL rather than silently switching back to upstream builds.
- Names forked packages `0.1.4-dtnpz.1` and reserves the `1000+` versionCode range for canary packages.
- Keeps the original module ID `duck-toolbox` so the fork replaces the upstream module rather than installing a second competing copy.
- Leaves the RKP signing and private-key export implementation unmodified; a prerequisite pass is **not** a certificate issuance guarantee.

## On Redmi Note 9 Pro / joyeuse

The Snapdragon 720G is a Qualcomm SoC, but that alone says nothing about the Qualcomm KeyMint vendor ABI or Google's RKP server acceptance. An Android 15 custom ROM can still depend on older vendor firmware.

Run the **read-only** diagnostic from a Linux host with ADB after installing the fork:

```sh
adb shell su -c 'sh /data/adb/modules/duck-toolbox/bin/joyeuse-diag.sh'
```

You can separately call the upstream read-only status command:

```sh
adb shell su -c '/data/adb/modules/duck-toolbox/bin/duckctl.sh rkp-device status'
```

This diagnostic does not invoke KeyMint signing commands, reach a server, generate private keys or install a keybox. Treat `prerequisites_met: true` only as *eligible to attempt*, not proof of compatibility.

## Build and releases

Run the existing GitHub Actions **CI** workflow. The `package` job produces a module artifact and, on main pushes, a canary prerelease ZIP. For stable releases, dispatch the upstream-derived **Stable** workflow on `main`; its release ZIP matches `update.json`.

## Installation and data

The fork keeps `id=duck-toolbox` and `/data/adb/duck-toolbox/var`. Save module settings before swapping upstream/fork builds. Do not install upstream and this fork simultaneously. Never share private key files or `keybox.xml` publicly.

## Credits and limits

Upstream: Eltavine and other upstream contributors. MIT: [LICENSE](../LICENSE). Changes here are not endorsed by the upstream project. The certificate issuer requires compatible firmware, and even a successful RKP certificate does **not** guarantee a Play Integrity verdict.

## Technical implementation and trust boundaries

The source implementation is documented in [RKP_ORACLE_TECHNIQUE.md](RKP_ORACLE_TECHNIQUE.md). The Qualcomm-specific V2 command is `0x3122`; the native legacy V1 command is `0x311e`. Neither Android version nor an available library proves the vendor ABI is compatible. The challenge, public key and TA response must be bound before submission, while the eventual X.509 chain must independently match the private key and verify cryptographically.

### Security fix added in this edition

The device issuer formerly sorted certificates by X.509 issuer/subject **names** and checked that the P-256 leaf matched the locally generated key. Those checks alone did not verify the issuer signatures or CA constraints. Its artifact path now also calls the already-existing `duck_rkp::certificates::select_chain_for_p256` verifier, which checks certificate signatures, expiry, CA/key usage, path length, duplicate/unrelated certificates and key binding **before writing key material**. Synthetic test certificates were updated to valid attestation CA chains, and corrupted-signature/non-CA fixtures are explicitly rejected. This is an offline validation hardening, not a claim that Google or the TEE will accept a CSR.
