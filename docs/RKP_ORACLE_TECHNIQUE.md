# Qualcomm KeyMint RKP signing oracle — protocol and source notes

This is a review of the public Telegram announcement (2026-10-16/17), the upstream implementation and AOSP RKP documentation. The [Kanxue write-up](https://bbs.kanxue.com/thread-293244.htm) was referenced, but its complete text was not accessible during this review. Never treat the Telegram announcement as independent confirmation of successful production issuance on a particular device.

## 1. The security boundary

- A normal Android **RKP** attestation key is created by KeyMint, and the normal HAL collects the certificate request.
- In the publicized oracle approach, an ordinary program creates an ECDSA P-256 keypair. Its **private key stays in normal-world software**, not in KeyMint.
- It asks a Qualcomm vendor KeyMint trusted application to authenticate a request containing the corresponding public key using the **device's DICE-backed identity**.
- If Google's RKP service accepts the CSR, the resulting certificate is for the *software-generated* key. It is not a TEE-origin private key, and successful issuance does not certify the software key's storage location.
- This is a vendor-firmware compatibility issue; a modern Android ROM running on legacy vendor firmware is not proof of support.

## 2. V2 transaction in current source

| Phase | Source | Detail |
| --- | --- | --- |
| Prerequisite read | `crates/duck-rkp-device/src/status.rs` | Requires Android arm64, root, Qualcomm SoC evidence, build fingerprint and at least one supported route. V2/native V1 need `/dev/smcinvoke` and `/vendor/lib64/libkeymasterdeviceutils.so`. |
| Keypair generation | `service.rs` / `duck-rkp/cose` | Fresh random P-256 software keypair; send only the public half for certification. |
| Fetch challenge | `service.rs` and `duck-rkp/http.rs` | `fetchEekChain` supplies challenge and EEK chains. Challenge is non-empty and bounded to 64 bytes. |
| Construct request | `protocol.rs::build_request` | Little-endian 4-byte command **`0x3122`**, followed by CBOR map **`{4: challenge, 57: bstr(CBOR [COSE_Key])}`**. COSE_Key uses kty=EC2 (2), alg=ES256 (-7), curve=P-256 (1), and 32-byte X and Y. |
| Vendor request | `keymint.rs` / `worker.rs` | Isolated subprocess dynamically loads `libkeymasterdeviceutils.so` and calls the Qualcomm `KeymasterUtils` native ABI. A crashing or incompatible TA must not crash the interactive daemon. 30-second worker bound. |
| Validate V2 CSR | `protocol.rs::validate_csr` | Accepts bare CSR or vendor field `61` wrapper. Validates signed DICE structure, challenge, public key, and single-key shape. |
| Certificate request | `service.rs` / `duck-rkp/http.rs` | Submit authenticated CSR through `signCertificates` to Google. Server may reject an otherwise valid CSR. |
| Validate result | `artifacts.rs::prepare` | Require exactly one bounded DER chain that matches generated P-256 key. The **dtnpz edition additionally enforces cryptographic signature and CA/path validation**, not only issuer-name linking. |
| Export | `artifacts.rs` | Atomically write private `keybox.xml`, `csr.cbor`, `cert_chain.der`, `private_key.pem`. Private files use mode 0600, directory 0700. |

## 3. V1 is a different cryptographic protocol

**Do not treat V1 as a V2 request with another command ID.**

- **V1 AIDL** uses a Qualcomm PureSoft (zero-HBK) specific signing oracle path, `app_process` and an available system RKP APK. The program builds the `MacedPublicKey` in Rust. DeviceInfo and ProtectedData must retain their native bytes for the server's AAD and decryption checks.
- **Native V1** uses Qualcomm vendor command **`0x311e`**, a fresh MAC key and the complete server-selected EEK chain. It shares the V1 CSR assembly with AIDL V1 but not the ABI entrypoint.
- V1's protected DICE evidence is encrypted for the RKP server. Local validation can only check the *structure*, not assert the encrypted evidence signature was valid.
- Automatic mode tries eligible **V2 → AIDL V1 → native V1** with a fresh server challenge for each failed method. Recorded failures are distinct by method and error code.

## 4. What must hold before an artifact is trusted

1. CSR's signed challenge must match the most recent RKP server challenge.
2. CSR's `keysToSign` must be the exact public half of the private key to export.
3. For V2, signed DICE and CSR evidence must pass cryptographic verification; for V1, only the server can confirm encrypted evidence.
4. Returned certificate must carry that public key; the chain must be a unique, complete path.
5. Each X.509 certificate signature, validity period, critical extension, CA authorization, key usage and issuer path length must be checked. **Issuer and subject text alone cannot establish a trusted signature.**
6. Successful local verification does not establish trusted Google root, revocation status, device enrollment, absence of abuse controls, or any Play Integrity verdict.
7. No secrets, raw keybox XML or PEM private keys should be exposed in public build logs or issue reports.

## 5. What is and isn't proven

- Upstream tests use synthetic keys and a local fake RKP server; they can verify serialization, binding and output safety, *not* successful real provisioning.
- The upstream README documents a OnePlus PJX110 experiment returning TA error `-18` for V2; merely loading the library is not sufficient.
- Firmware-dependent ABI may differ by Qualcomm generation. A `joyeuse` running Android 15 on an older proprietary vendor image may not implement either command; no success claim should be made without a consenting real-device test.
- The initial diagnostic reads hardware properties and status only. Provisioning is a separate user-approved operation.

## 6. Primary references

- [AOSP RKP README](https://android.googlesource.com/platform/hardware/interfaces/+/HEAD/security/rkp/README.md)
- [AOSP KeyMint RPC documentation](https://android.googlesource.com/platform/hardware/interfaces/+/f75880001f/security/keymint/RKP_README.md)
- [Upstream Duck ToolBox](https://github.com/eltavine/Duck-ToolBox)
- [Kanxue write-up reference](https://bbs.kanxue.com/thread-293244.htm)
