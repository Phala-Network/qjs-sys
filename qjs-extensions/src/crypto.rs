use js::{Result, ToJsValue};
use rand::RngCore;

fn from_js<T>(value: js::Value) -> Result<T>
where
    T: js::FromJsValue,
{
    T::from_js_value(value)
}

macro_rules! js_bail {
    ($($arg:tt)*) => {
        return Err(js::Error::Custom(format!($($arg)*)))
    };
}

#[derive(js::FromJsValue, Debug)]
struct BaseAlgorithm {
    name: js::JsString,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjsbind(rename_all = "camelCase")]
struct RsaOaepParams {
    label: js::Bytes,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjsbind(rename_all = "camelCase")]
struct AesCtrParams {
    counter: js::Bytes,
    length: usize,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjsbind(rename_all = "camelCase")]
struct AesCbcParams {
    iv: js::Bytes,
}

#[derive(js::FromJsValue, Debug)]
#[qjsbind(rename_all = "camelCase")]
struct AesGcmParams {
    iv: js::Bytes,
    additional_data: Option<js::Bytes>,
    tag_length: Option<usize>,
}

enum CryptAlgorithm {
    RsaOaep(RsaOaepParams),
    AesCtr(AesCtrParams),
    AesCbc(AesCbcParams),
    AesGcm(AesGcmParams),
}

impl js::FromJsValue for CryptAlgorithm {
    fn from_js_value(value: js::Value) -> Result<Self> {
        use CryptAlgorithm::*;
        let base = BaseAlgorithm::from_js_value(value.clone())?;
        match base.name.as_str() {
            "AES-GCM" => Ok(AesGcm(from_js(value)?)),
            "AES-CBC" => Ok(AesCbc(from_js(value)?)),
            "AES-CTR" => Ok(AesCtr(from_js(value)?)),
            "RSA-OAEP" => Ok(RsaOaep(from_js(value)?)),
            _ => js_bail!("unsupported algorithm: {}", base.name),
        }
    }
}

#[derive(js::FromJsValue)]
#[qjsbind(rename_all = "camelCase")]
struct EcdhKeyDeriveParams {
    public: CryptoKey,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjsbind(rename_all = "camelCase")]
struct HkdfParams {
    hash: js::JsString,
    salt: js::Bytes,
    info: js::Bytes,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjsbind(rename_all = "camelCase")]
struct Pbkdf2Params {
    hash: js::JsString,
    salt: js::Bytes,
    iterations: usize,
}

enum DeriveAlgorithm {
    Ecdh(EcdhKeyDeriveParams),
    Hkdf(HkdfParams),
    Pbkdf2(Pbkdf2Params),
}

impl js::FromJsValue for DeriveAlgorithm {
    fn from_js_value(value: js::Value) -> Result<Self> {
        use DeriveAlgorithm::*;
        let base = BaseAlgorithm::from_js_value(value.clone())?;
        match base.name.as_str() {
            "ECDH" => Ok(Ecdh(from_js(value)?)),
            "HKDF" => Ok(Hkdf(from_js(value)?)),
            "PBKDF2" => Ok(Pbkdf2(from_js(value)?)),
            _ => js_bail!("unsupported algorithm: {}", base.name),
        }
    }
}

#[derive(js::FromJsValue, js::ToJsValue, Debug, Clone)]
#[qjsbind(rename_all = "camelCase")]
struct HmacKeyGenParams {
    hash: js::JsString,
    length: Option<usize>,
}

#[derive(js::FromJsValue, js::ToJsValue, Debug, Clone)]
struct AesKeyGenParams {
    name: js::JsString,
    length: usize,
}

enum DeriveKeyGenAlgorithm {
    Hmac(HmacKeyGenParams),
    Aes(AesKeyGenParams),
    Hkdf(HkdfParams),
    Pbkdf2(Pbkdf2Params),
}

impl js::FromJsValue for DeriveKeyGenAlgorithm {
    fn from_js_value(value: js::Value) -> Result<Self> {
        use DeriveKeyGenAlgorithm::*;
        let base = BaseAlgorithm::from_js_value(value.clone())?;
        match base.name.as_str() {
            "HMAC" => Ok(Hmac(from_js(value)?)),
            "AES-CBC" | "AES-CTR" | "AES-GCM" | "AES-KW" => Ok(Aes(from_js(value)?)),
            "HKDF" => Ok(Hkdf(from_js(value)?)),
            "PBKDF2" => Ok(Pbkdf2(from_js(value)?)),
            _ => js_bail!("unsupported algorithm: {}", base.name),
        }
    }
}

#[derive(js::FromJsValue, js::ToJsValue, Debug, Clone)]
#[qjsbind(rename_all = "camelCase")]
struct RsaHashedKeyGenParams {
    name: js::JsString,
    modulus_length: usize,
    public_exponent: js::Bytes,
    hash: js::JsString,
}

#[derive(js::FromJsValue, js::ToJsValue, Debug, Clone)]
#[qjsbind(rename_all = "camelCase")]
struct EcKeyGenParams {
    name: js::JsString,
    named_curve: js::JsString,
}

#[derive(Clone)]
enum KeyGenAlgorithm {
    Rsa(RsaHashedKeyGenParams),
    Ec(EcKeyGenParams),
    Hmac(HmacKeyGenParams),
    Aes(AesKeyGenParams),
}

impl js::FromJsValue for KeyGenAlgorithm {
    fn from_js_value(value: js::Value) -> Result<Self> {
        use KeyGenAlgorithm::*;
        let base = BaseAlgorithm::from_js_value(value.clone())?;
        match base.name.as_str() {
            "RSASSA-PKCS1-v1_5" | "RSA-OAEP" | "RSA-PSS" => Ok(Rsa(from_js(value)?)),
            "ECDSA" | "ECDH" => Ok(Ec(from_js(value)?)),
            "HMAC" => Ok(Hmac(from_js(value)?)),
            "AES-CBC" | "AES-CTR" | "AES-GCM" | "AES-KW" => Ok(Aes(from_js(value)?)),
            _ => js_bail!("unsupported algorithm: {}", base.name),
        }
    }
}

impl js::ToJsValue for KeyGenAlgorithm {
    fn to_js_value(&self, ctx: &js::Context) -> Result<js::Value> {
        match self {
            KeyGenAlgorithm::Rsa(params) => params.to_js_value(ctx),
            KeyGenAlgorithm::Ec(params) => params.to_js_value(ctx),
            KeyGenAlgorithm::Hmac(params) => params.to_js_value(ctx),
            KeyGenAlgorithm::Aes(params) => params.to_js_value(ctx),
        }
    }
}

#[derive(js::FromJsValue, ToJsValue)]
#[qjsbind(rename_all = "camelCase")]
struct CryptoKey {
    r#type: String,
    extractable: bool,
    algorithm: KeyGenAlgorithm,
    usages: Vec<js::JsString>,
    _raw: js::Bytes,
}

#[derive(js::FromJsValue, js::ToJsValue)]
#[qjsbind(rename_all = "camelCase")]
struct CryptoKeyPair {
    public_key: CryptoKey,
    private_key: CryptoKey,
}

enum CryptoKeyOrPair {
    #[allow(dead_code)]
    Key(CryptoKey),
    Pair(CryptoKeyPair),
}

impl CryptoKeyOrPair {
    fn from_pair_raw(
        priviate_key: js::Bytes,
        public_key: js::Bytes,
        extractable: bool,
        usages: Vec<js::JsString>,
        algorithm: KeyGenAlgorithm,
    ) -> Self {
        let public_key = CryptoKey {
            r#type: "public".into(),
            extractable,
            usages: usages.clone(),
            algorithm: algorithm.clone(),
            _raw: public_key,
        };
        let private_key = CryptoKey {
            r#type: "private".into(),
            extractable,
            usages: usages,
            algorithm,
            _raw: priviate_key,
        };
        CryptoKeyOrPair::Pair(CryptoKeyPair {
            public_key,
            private_key,
        })
    }
}

impl js::ToJsValue for CryptoKeyOrPair {
    fn to_js_value(&self, ctx: &js::Context) -> Result<js::Value> {
        match self {
            CryptoKeyOrPair::Key(key) => key.to_js_value(ctx),
            CryptoKeyOrPair::Pair(pair) => pair.to_js_value(ctx),
        }
    }
}

#[js::host_call]
fn encrypt(
    algorithm: CryptAlgorithm,
    key: CryptoKey,
    data: js::BytesOrString,
) -> Result<js::Bytes> {
    match algorithm {
        CryptAlgorithm::AesGcm(params) => {
            use aes::cipher::consts::U12;
            use aes_gcm::aead::Aead;
            use aes_gcm::KeyInit;

            macro_rules! encrypt_with {
                ($key_size:ident) => {{
                    let aead =
                        aes_gcm::AesGcm::<aes::$key_size, U12>::new(
                            aes_gcm::Key::<aes::$key_size>::from_slice(&key._raw),
                        );
                    let nonce = aes_gcm::Nonce::from_slice(&params.iv);
                    let ciphertext = aead
                        .encrypt(nonce, data.as_ref())
                        .map_err(|_| js::Error::Static("encryption failed"))?;
                    ciphertext
                }};
            }
            if params.additional_data.is_some() {
                js_bail!("Additional data is not supported");
            }
            if params.tag_length.is_some() {
                js_bail!("Tag length is not supported");
            }
            if key.r#type.as_str() != "secret" {
                js_bail!("Key must be a secret key");
            }
            if params.iv.len() != 12 {
                js_bail!("IV must be 12 bytes long");
            }
            let ciphertext = match key._raw.len() {
                16 => encrypt_with!(Aes128),
                24 => encrypt_with!(Aes192),
                32 => encrypt_with!(Aes256),
                _ => js_bail!("Key must be 16, 24, or 32 bytes long"),
            };
            Ok(ciphertext.into())
        }
        _ => js_bail!("Unsupported encryption algorithm"),
    }
}

#[js::host_call]
fn decrypt(
    algorithm: CryptAlgorithm,
    key: CryptoKey,
    data: js::BytesOrString,
) -> Result<js::Bytes> {
    match algorithm {
        CryptAlgorithm::AesGcm(params) => {
            use aes::cipher::consts::U12;
            use aes_gcm::aead::Aead;
            use aes_gcm::KeyInit;
            macro_rules! decrypt_with {
                ($key_size:ident) => {{
                    let aead =
                        aes_gcm::AesGcm::<aes::$key_size, U12>::new(
                            aes_gcm::Key::<aes::$key_size>::from_slice(&key._raw),
                        );
                    let nonce = aes_gcm::Nonce::from_slice(&params.iv);
                    let plaintext = aead
                        .decrypt(nonce, data.as_ref())
                        .map_err(|_| js::Error::Static("decryption failed"))?;
                    plaintext
                }};
            }
            if params.additional_data.is_some() {
                js_bail!("Additional data is not supported");
            }
            if params.tag_length.is_some() {
                js_bail!("Tag length is not supported");
            }
            if params.iv.len() != 12 {
                js_bail!("IV must be 12 bytes long");
            }
            let plaintext = match key._raw.len() {
                16 => decrypt_with!(Aes128),
                24 => decrypt_with!(Aes192),
                32 => decrypt_with!(Aes256),
                _ => js_bail!("Key must be 16, 24, or 32 bytes long"),
            };
            Ok(plaintext.into())
        }
        _ => js_bail!("Unsupported decryption algorithm"),
    }
}

fn derive_aes_key(
    shared_secret: impl AsRef<[u8]>,
    derived_key_algorithm: DeriveKeyGenAlgorithm,
    extractable: bool,
    key_usages: Vec<js::JsString>,
) -> Result<CryptoKey> {
    if let DeriveKeyGenAlgorithm::Aes(aes_params) = derived_key_algorithm {
        let shared_secret_bytes = shared_secret.as_ref();
        // Use the shared secret to generate AES key
        let key_len = aes_params.length / 8;
        let Some(derived_key) = &shared_secret_bytes.get(..key_len) else {
            js_bail!("Shared secret is too short");
        };
        Ok(CryptoKey {
            r#type: "secret".into(),
            extractable,
            algorithm: KeyGenAlgorithm::Aes(aes_params),
            usages: key_usages,
            _raw: derived_key.to_vec().into(),
        })
    } else {
        js_bail!("Unsupported derived key algorithm")
    }
}

#[js::host_call]
fn derive_key(
    algorithm: DeriveAlgorithm,
    base_key: CryptoKey,
    derived_key_algorithm: DeriveKeyGenAlgorithm,
    extractable: bool,
    key_usages: Vec<js::JsString>,
) -> Result<CryptoKey> {
    match algorithm {
        DeriveAlgorithm::Ecdh(params) => {
            let KeyGenAlgorithm::Ec(base_algo) = &base_key.algorithm else {
                js_bail!("Unsupported base key algorithm");
            };
            macro_rules! derive_aes_key {
                ($module: ident, $curve: ident) => {{
                    use $module::{
                        ecdh::diffie_hellman, elliptic_curve::SecretKey, $curve, PublicKey,
                    };
                    // Process keys
                    let secret_key = SecretKey::<$curve>::from_slice(&base_key._raw)
                        .map_err(|_| js::Error::Static("Invalid private key"))?;
                    let public_key = PublicKey::from_sec1_bytes(&params.public._raw.to_vec())
                        .map_err(|_| js::Error::Static("Invalid public key"))?;
                    // Perform ECDH & derive key
                    let shared_secret =
                        diffie_hellman(secret_key.to_nonzero_scalar(), public_key.as_affine());
                    derive_aes_key(
                        shared_secret.raw_secret_bytes(),
                        derived_key_algorithm,
                        extractable,
                        key_usages,
                    )
                }};
            }
            match base_algo.named_curve.as_str() {
                "P-256" => derive_aes_key!(p256, NistP256),
                "P-384" => derive_aes_key!(p384, NistP384),
                "P-521" => derive_aes_key!(p521, NistP521),
                _ => js_bail!(
                    "Unsupported named curve: {}",
                    base_algo.named_curve.as_str()
                ),
            }
        }
        _ => js_bail!("Unsupported derive algorithm"),
    }
}

#[js::host_call]
fn generate_key(
    algorithm: KeyGenAlgorithm,
    extractable: bool,
    key_usages: Vec<js::JsString>,
) -> Result<CryptoKeyOrPair> {
    use p256::elliptic_curve::sec1::ToEncodedPoint;
    use p256::SecretKey as SecretKeyP256;
    use p384::SecretKey as SecretKeyP384;
    use p521::SecretKey as SecretKeyP521;

    match &algorithm {
        KeyGenAlgorithm::Ec(params) => match params.named_curve.as_str() {
            "P-256" => {
                let secret_key = SecretKeyP256::random(&mut rand::rngs::OsRng);
                let public_key = secret_key.public_key();

                let private_key_bytes = secret_key.to_bytes().to_vec();
                let public_key_bytes = public_key.to_encoded_point(false).as_bytes().to_vec();
                Ok(CryptoKeyOrPair::from_pair_raw(
                    private_key_bytes.into(),
                    public_key_bytes.into(),
                    extractable,
                    key_usages,
                    algorithm,
                ))
            }
            "P-384" => {
                let secret_key = SecretKeyP384::random(&mut rand::rngs::OsRng);
                let public_key = secret_key.public_key();

                let private_key_bytes = secret_key.to_bytes().to_vec();
                let public_key_bytes = public_key.to_encoded_point(false).as_bytes().to_vec();
                Ok(CryptoKeyOrPair::from_pair_raw(
                    private_key_bytes.into(),
                    public_key_bytes.into(),
                    extractable,
                    key_usages,
                    algorithm,
                ))
            }
            "P-521" => {
                let secret_key = SecretKeyP521::random(&mut rand::rngs::OsRng);
                let public_key = secret_key.public_key();

                let private_key_bytes = secret_key.to_bytes().to_vec();
                let public_key_bytes = public_key.to_encoded_point(false).as_bytes().to_vec();

                Ok(CryptoKeyOrPair::from_pair_raw(
                    private_key_bytes.into(),
                    public_key_bytes.into(),
                    extractable,
                    key_usages,
                    algorithm,
                ))
            }
            _ => js_bail!("Unsupported named curve: {}", params.named_curve),
        },
        _ => js_bail!("Unsupported key generation algorithm"),
    }
}

#[js::host_call]
fn import_key(
    fmt: js::JsString,
    key_data: js::Value,
    algorithm: KeyGenAlgorithm,
    extractable: bool,
    key_usages: Vec<js::JsString>,
) -> Result<CryptoKey> {
    if fmt.as_str() != "raw" {
        js_bail!("Unsupported import format: {fmt}");
    }
    use js::FromJsValue;
    let key_data = js::Bytes::from_js_value(key_data)?;
    Ok(CryptoKey {
        r#type: "secret".into(),
        extractable,
        algorithm,
        usages: key_usages,
        _raw: key_data,
    })
}

#[js::host_call]
fn export_key(fmt: js::JsString, key: CryptoKey) -> Result<js::Bytes> {
    if fmt.as_str() != "raw" {
        js_bail!("Unsupported export format: {fmt}");
    }
    Ok(key._raw.clone())
}

#[js::host_call]
fn get_random_values(output: js::JsUint8Array) -> Result<js::JsUint8Array> {
    let mut buf = vec![0u8; output.len()];
    rand::thread_rng().fill_bytes(&mut buf);
    output.fill_with_bytes(&buf);
    Ok(output)
}

#[js::host_call]
fn random_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn setup_subtle(ns: &js::Value) -> Result<()> {
    ns.define_property_fn("encrypt", encrypt)?;
    ns.define_property_fn("decrypt", decrypt)?;
    ns.define_property_fn("deriveKey", derive_key)?;
    ns.define_property_fn("generateKey", generate_key)?;
    ns.define_property_fn("importKey", import_key)?;
    ns.define_property_fn("exportKey", export_key)?;
    Ok(())
}

pub fn setup(g: &js::Value) -> Result<()> {
    let crypto = js::Value::new_object(g.context()?);
    let subtle = js::Value::new_object(g.context()?);
    setup_subtle(&subtle)?;
    crypto.set_property("subtle", &subtle)?;
    crypto.define_property_fn("getRandomValues", get_random_values)?;
    crypto.define_property_fn("randomUUID", random_uuid)?;
    g.set_property("crypto", &crypto)?;
    Ok(())
}
