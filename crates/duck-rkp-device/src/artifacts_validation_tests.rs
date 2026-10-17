//! Certificate verification regressions: deliberately invalid issuer evidence.
use super::*;

#[test]
fn rejects_a_tampered_leaf_signature_without_publishing_any_keybox() {
    let (secret, mut leaf, root) = two_level_chain();
    // Changing a signature byte preserves DER structure and the public key but
    // invalidates the cryptographic issuer-to-leaf link.
    let last = leaf.len() - 1;
    leaf[last] ^= 1;
    let stream = [leaf, root].concat();
    let dir = tempfile::tempdir().unwrap();
    let paths = AppPaths::for_root(dir.path(), dir.path());
    let error = write(
        &paths,
        Inputs {
            secret_key: &secret,
            csr: &[1, 2, 3],
            cert_chain_der: &stream,
            manufacturer: "QTI",
        },
    )
    .unwrap_err();
    assert_eq!(crate::error::code_of(&error), "invalid_device_csr");
    assert!(error.to_string().contains("certificate path verification"));
    let output_count = fs::read_dir(&paths.outputs_dir)
        .map(|entries| entries.count())
        .unwrap_or(0);
    assert_eq!(output_count, 0, "a rejected signature must not publish a keybox");
}

#[test]
fn rejects_leaf_without_ca_authorization() {
    use rcgen::CertifiedIssuer;
    let mut root_params = params("root");
    root_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    root_params.key_usages = vec![KeyUsagePurpose::KeyCertSign];
    let root = CertifiedIssuer::self_signed(
        root_params,
        KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).unwrap(),
    )
    .unwrap();
    let key = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).unwrap();
    let secret = P256SecretKey::from_pkcs8_der(key.serialized_der()).unwrap();
    let leaf = params("non-ca-leaf").signed_by(&key, &root).unwrap();
    let stream = [leaf.der().as_ref(), root.der().as_ref()].concat();
    let dir = tempfile::tempdir().unwrap();
    let paths = AppPaths::for_root(dir.path(), dir.path());
    let error = write(
        &paths,
        Inputs {
            secret_key: &secret,
            csr: &[1, 2, 3],
            cert_chain_der: &stream,
            manufacturer: "QTI",
        },
    )
    .unwrap_err();
    assert_eq!(crate::error::code_of(&error), "invalid_device_csr");
    assert!(!paths.outputs_dir.exists());
}
