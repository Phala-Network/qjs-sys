use anyhow::bail;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use rand::RngCore;

use cipher::generic_array::GenericArray;
use cipher::{ArrayLength, KeyInit, StreamCipher};

use js::{NoStdContext, Native, Result, ToJsValue};

fn from_js<T>(value: js::Value) -> Result<T>
where
    T: js::FromJsValue,
{
    T::from_js_value(value)
}

#[derive(js::FromJsValue, Debug)]
struct BaseAlgorithm {
    name: js::JsString,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjs(rename_all = "camelCase")]
struct RsaOaepParams {
    label: js::Bytes,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjs(rename_all = "camelCase")]
struct AesCtrParams {
    counter: js::Bytes,
    length: usize,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjs(rename_all = "camelCase")]
struct AesCbcParams {
    iv: js::Bytes,
}

#[derive(js::FromJsValue, Debug)]
#[qjs(rename_all = "camelCase")]
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
            _ => bail!("unsupported algorithm: {}", base.name),
        }
    }
}

#[derive(js::FromJsValue)]
#[qjs(rename_all = "camelCase")]
struct EcdhKeyDeriveParams {
    public: Native<CryptoKey>,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjs(rename_all = "camelCase")]
struct HkdfParams {
    hash: js::JsString,
    salt: js::Bytes,
    info: js::Bytes,
}

#[allow(dead_code)]
#[derive(js::FromJsValue, Debug)]
#[qjs(rename_all = "camelCase")]
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
            _ => bail!("unsupported algorithm: {}", base.name),
        }
    }
}

#[derive(js::FromJsValue, js::ToJsValue, js::GcMark, Debug, Clone)]
#[qjs(rename_all = "camelCase")]
struct HmacKeyGenParams {
    hash: js::JsString,
    length: Option<usize>,
}

#[derive(js::FromJsValue, js::ToJsValue, js::GcMark, Debug, Clone)]
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
            _ => bail!("unsupported algorithm: {}", base.name),
        }
    }
}

#[derive(js::FromJsValue, js::ToJsValue, js::GcMark, Debug, Clone)]
#[qjs(rename_all = "camelCase")]
struct RsaHashedKeyGenParams {
    name: js::JsString,
    modulus_length: usize,
    public_exponent: js::Bytes,
    hash: js::JsString,
}

#[derive(js::FromJsValue, js::ToJsValue, js::GcMark, Debug, Clone)]
#[qjs(rename_all = "camelCase")]
struct EcKeyGenParams {
    name: js::JsString,
    named_curve: js::JsString,
}

#[derive(Clone, js::GcMark)]
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
            _ => bail!("unsupported algorithm: {}", base.name),
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

use native_classes::CryptoKey;

#[js::qjsbind]
mod native_classes {
    use super::{KeyGenAlgorithm, String, Vec};

    #[qjs(class(rename_all = "camelCase"))]
    pub struct CryptoKey {
        #[qjs(getter)]
        pub r#type: String,
        #[qjs(getter)]
        pub extractable: bool,
        #[qjs(getter)]
        pub algorithm: KeyGenAlgorithm,
        #[qjs(getter)]
        pub usages: Vec<js::JsString>,
        pub raw: Vec<u8>,
    }
}

#[derive(ToJsValue)]
#[qjs(rename_all = "camelCase")]
struct CryptoKeyPair {
    public_key: Native<CryptoKey>,
    private_key: Native<CryptoKey>,
}

enum CryptoKeyOrPair {
    #[allow(dead_code)]
    Key(Native<CryptoKey>),
    Pair(CryptoKeyPair),
}

impl ToJsValue for CryptoKeyOrPair {
    fn to_js_value(&self, ctx: &js::Context) -> Result<js::Value> {
        match self {
            CryptoKeyOrPair::Key(key) => key.to_js_value(ctx),
            CryptoKeyOrPair::Pair(pair) => pair.to_js_value(ctx),
        }
    }
}

impl CryptoKeyOrPair {
    fn from_pair_raw(
        ctx: js::Context,
        priviate_key: Vec<u8>,
        public_key: Vec<u8>,
        extractable: bool,
        usages: Vec<js::JsString>,
        algorithm: KeyGenAlgorithm,
    ) -> js::Result<Self> {
        let public_key = CryptoKey {
            r#type: "public".into(),
            extractable: true,
            usages: usages.clone(),
            algorithm: algorithm.clone(),
            raw: public_key,
        };
        let private_key = CryptoKey {
            r#type: "private".into(),
            extractable,
            usages: usages,
            algorithm,
            raw: priviate_key,
        };
        Ok(CryptoKeyOrPair::Pair(CryptoKeyPair {
            public_key: Native::new(&ctx, public_key)?,
            private_key: Native::new(&ctx, private_key)?,
        }))
    }
}

fn generic_array_from_slice<L>(arr: &[u8]) -> Result<GenericArray<u8, L>>
where
    L: ArrayLength<u8>,
{
    GenericArray::from_exact_iter(arr.iter().copied()).context("invalid length")
}

#[js::host_call]
fn encrypt(
    algorithm: CryptAlgorithm,
    key: Native<CryptoKey>,
    data: js::BytesOrString,
) -> Result<js::Bytes> {
    let key = key.borrow();
    match algorithm {
        CryptAlgorithm::AesGcm(params) => {
            use aes::cipher::consts::U12;
            use aes_gcm::aead::Aead;
            use aes_gcm::KeyInit;
            macro_rules! encrypt_with {
                ($key_size:ident) => {{
                    let aead = aes_gcm::AesGcm::<aes::$key_size, U12>::new(
                        &generic_array_from_slice(&key.raw).context("invalid key length")?,
                    );
                    let nonce = generic_array_from_slice(&params.iv)?;
                    let ciphertext = aead
                        .encrypt(&nonce, data.as_ref())
                        .context("encryption failed")?;
                    ciphertext
                }};
            }
            if params.additional_data.is_some() {
                bail!("additional data is not supported");
            }
            if params.tag_length.is_some() {
                bail!("tag length is not supported");
            }
            if key.r#type.as_str() != "secret" {
                bail!("key must be a secret key");
            }
            if params.iv.len() != 12 {
                bail!("iv must be 12 bytes long");
            }
            let KeyGenAlgorithm::Aes(key_algo) = &key.algorithm else {
                bail!("not a valid AES key algorithm");
            };
            let ciphertext = match key_algo.length {
                128 => encrypt_with!(Aes128),
                192 => encrypt_with!(Aes192),
                256 => encrypt_with!(Aes256),
                _ => bail!("key must be 16, 24, or 32 bytes long"),
            };
            Ok(ciphertext.into())
        }
        CryptAlgorithm::AesCbc(params) => {
            use aes::cipher::{block_padding::Pkcs7, BlockCipher, BlockEncryptMut, KeyIvInit};
            use aes::{Aes128, Aes192, Aes256};
            use cbc::Encryptor;
            fn encrypt_with<C>(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>>
            where
                C: BlockEncryptMut + BlockCipher + KeyInit,
            {
                let key = generic_array_from_slice(key).context("invalid key length")?;
                let iv = generic_array_from_slice(iv).context("invalid iv length")?;
                let cipher = Encryptor::<C>::new(&key, &iv);
                Ok(cipher.encrypt_padded_vec_mut::<Pkcs7>(data))
            }
            let KeyGenAlgorithm::Aes(key_algo) = &key.algorithm else {
                bail!("not a valid AES key algorithm");
            };
            let ciphertext = match key_algo.length {
                128 => encrypt_with::<Aes128>(&key.raw, &params.iv, data.as_bytes())?,
                192 => encrypt_with::<Aes192>(&key.raw, &params.iv, data.as_bytes())?,
                256 => encrypt_with::<Aes256>(&key.raw, &params.iv, data.as_bytes())?,
                _ => bail!("key must be 16, 24, or 32 bytes long"),
            };
            Ok(ciphertext.into())
        }
        CryptAlgorithm::AesCtr(params) => {
            use aes::cipher::KeyIvInit;
            use ctr::Ctr64LE;
            macro_rules! encrypt_with {
                ($key_size:ident) => {{
                    let mut cipher = Ctr64LE::<aes::$key_size>::new(
                        &generic_array_from_slice(&key.raw).context("invalid key length")?,
                        &generic_array_from_slice(&params.counter)
                            .context("invalid counter length")?,
                    );
                    let mut data = data.as_bytes().to_vec();
                    cipher
                        .try_apply_keystream(&mut data)
                        .context("encryption failed")?;
                    data
                }};
            }
            let KeyGenAlgorithm::Aes(key_algo) = &key.algorithm else {
                bail!("not a valid AES key algorithm");
            };
            let ciphertext = match key_algo.length {
                128 => encrypt_with!(Aes128),
                192 => encrypt_with!(Aes192),
                256 => encrypt_with!(Aes256),
                _ => bail!("key must be 16, 24, or 32 bytes long"),
            };
            Ok(ciphertext.into())
        }
        _ => bail!("unsupported encryption algorithm"),
    }
}

#[js::host_call]
fn decrypt(
    algorithm: CryptAlgorithm,
    key: Native<CryptoKey>,
    data: js::BytesOrString,
) -> Result<js::Bytes> {
    let key = key.borrow();
    match algorithm {
        CryptAlgorithm::AesGcm(params) => {
            use aes::cipher::consts::U12;
            use aes_gcm::aead::Aead;
            use aes_gcm::KeyInit;
            macro_rules! decrypt_with {
                ($key_size:ident) => {{
                    let aead = aes_gcm::AesGcm::<aes::$key_size, U12>::new(
                        &generic_array_from_slice(&key.raw)?,
                    );
                    let nonce = generic_array_from_slice(&params.iv)?;
                    let plaintext = aead
                        .decrypt(&nonce, data.as_ref())
                        .context("decryption failed")?;
                    plaintext
                }};
            }
            if params.additional_data.is_some() {
                bail!("additional data is not supported");
            }
            if params.tag_length.is_some() {
                bail!("tag length is not supported");
            }
            if params.iv.len() != 12 {
                bail!("iv must be 12 bytes long");
            }
            let KeyGenAlgorithm::Aes(key_algo) = &key.algorithm else {
                bail!("not a valid AES key algorithm");
            };
            let plaintext = match key_algo.length {
                128 => decrypt_with!(Aes128),
                192 => decrypt_with!(Aes192),
                256 => decrypt_with!(Aes256),
                _ => bail!("key must be 16, 24, or 32 bytes long"),
            };
            Ok(plaintext.into())
        }
        CryptAlgorithm::AesCbc(params) => {
            use aes::cipher::{block_padding::Pkcs7, BlockCipher, BlockDecryptMut, KeyIvInit};
            use aes::{Aes128, Aes192, Aes256};
            use cbc::Decryptor;
            fn decrypt_with<C>(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>>
            where
                C: BlockDecryptMut + BlockCipher + KeyInit,
            {
                let key = generic_array_from_slice(key).context("invalid key length")?;
                let iv = generic_array_from_slice(iv).context("invalid iv length")?;
                let cipher = Decryptor::<C>::new(&key, &iv);
                Ok(cipher
                    .decrypt_padded_vec_mut::<Pkcs7>(data)
                    .context("failed to decrypt")?)
            }
            let KeyGenAlgorithm::Aes(key_algo) = &key.algorithm else {
                bail!("not a valid AES key algorithm");
            };
            let plaintext = match key_algo.length {
                128 => decrypt_with::<Aes128>(&key.raw, &params.iv, data.as_bytes())?,
                192 => decrypt_with::<Aes192>(&key.raw, &params.iv, data.as_bytes())?,
                256 => decrypt_with::<Aes256>(&key.raw, &params.iv, data.as_bytes())?,
                _ => bail!("key must be 16, 24, or 32 bytes long"),
            };
            Ok(plaintext.into())
        }
        CryptAlgorithm::AesCtr(params) => {
            use aes::cipher::KeyIvInit;
            use ctr::Ctr64LE;
            macro_rules! decrypt_with {
                ($key_size:ident) => {{
                    let mut cipher = Ctr64LE::<aes::$key_size>::new(
                        &generic_array_from_slice(&key.raw).context("invalid key length")?,
                        &generic_array_from_slice(&params.counter)
                            .context("invalid counter length")?,
                    );
                    let mut data = data.as_bytes().to_vec();
                    cipher
                        .try_apply_keystream(&mut data)
                        .context("decryption failed")?;
                    data
                }};
            }
            let KeyGenAlgorithm::Aes(key_algo) = &key.algorithm else {
                bail!("not a valid AES key algorithm");
            };
            let plaintext = match key_algo.length {
                128 => decrypt_with!(Aes128),
                192 => decrypt_with!(Aes192),
                256 => decrypt_with!(Aes256),
                _ => bail!("key must be 16, 24, or 32 bytes long"),
            };
            Ok(plaintext.into())
        }
        _ => bail!("unsupported decryption algorithm"),
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
            bail!("shared secret is too short");
        };
        Ok(CryptoKey {
            r#type: "secret".into(),
            extractable,
            algorithm: KeyGenAlgorithm::Aes(aes_params),
            usages: key_usages,
            raw: derived_key.to_vec().into(),
        })
    } else {
        bail!("unsupported derived key algorithm")
    }
}

#[js::host_call(with_context)]
fn derive_key(
    ctx: js::Context,
    _this_value: js::Value,
    algorithm: DeriveAlgorithm,
    base_key: Native<CryptoKey>,
    derived_key_algorithm: DeriveKeyGenAlgorithm,
    extractable: bool,
    key_usages: Vec<js::JsString>,
) -> Result<Native<CryptoKey>> {
    let base_key = base_key.borrow();
    let key = match algorithm {
        DeriveAlgorithm::Ecdh(params) => {
            let KeyGenAlgorithm::Ec(base_algo) = &base_key.algorithm else {
                bail!("unsupported base key algorithm");
            };
            macro_rules! derive_aes_key {
                ($module: ident, $curve: ident) => {{
                    use $module::{
                        ecdh::diffie_hellman, elliptic_curve::SecretKey, $curve, PublicKey,
                    };
                    // Process keys
                    let secret_key = SecretKey::<$curve>::from_slice(&base_key.raw)
                        .context("invalid private key")?;
                    let public_key =
                        PublicKey::from_sec1_bytes(&params.public.borrow().raw.to_vec())
                            .context("invalid public key")?;
                    // Perform ECDH & derive key
                    let shared_secret =
                        diffie_hellman(secret_key.to_nonzero_scalar(), public_key.as_affine());
                    derive_aes_key(
                        shared_secret.raw_secret_bytes(),
                        derived_key_algorithm,
                        extractable,
                        key_usages,
                    )?
                }};
            }
            match base_algo.named_curve.as_str() {
                "P-256" => derive_aes_key!(p256, NistP256),
                "P-384" => derive_aes_key!(p384, NistP384),
                "P-521" => derive_aes_key!(p521, NistP521),
                _ => bail!(
                    "unsupported named curve: {}",
                    base_algo.named_curve.as_str()
                ),
            }
        }
        _ => bail!("unsupported derive algorithm"),
    };
    Native::new(&ctx, key)
}

#[js::host_call(with_context)]
fn generate_key(
    ctx: js::Context,
    _this: js::Value,
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
                CryptoKeyOrPair::from_pair_raw(
                    ctx,
                    private_key_bytes,
                    public_key_bytes,
                    extractable,
                    key_usages,
                    algorithm,
                )
            }
            "P-384" => {
                let secret_key = SecretKeyP384::random(&mut rand::rngs::OsRng);
                let public_key = secret_key.public_key();

                let private_key_bytes = secret_key.to_bytes().to_vec();
                let public_key_bytes = public_key.to_encoded_point(false).as_bytes().to_vec();
                CryptoKeyOrPair::from_pair_raw(
                    ctx,
                    private_key_bytes.into(),
                    public_key_bytes.into(),
                    extractable,
                    key_usages,
                    algorithm,
                )
            }
            "P-521" => {
                let secret_key = SecretKeyP521::random(&mut rand::rngs::OsRng);
                let public_key = secret_key.public_key();

                let private_key_bytes = secret_key.to_bytes().to_vec();
                let public_key_bytes = public_key.to_encoded_point(false).as_bytes().to_vec();

                CryptoKeyOrPair::from_pair_raw(
                    ctx,
                    private_key_bytes.into(),
                    public_key_bytes.into(),
                    extractable,
                    key_usages,
                    algorithm,
                )
            }
            _ => bail!("unsupported named curve: {}", params.named_curve),
        },
        _ => bail!("unsupported key generation algorithm"),
    }
}

#[js::host_call(with_context)]
fn import_key(
    ctx: js::Context,
    _this: js::Value,
    fmt: js::JsString,
    key_data: js::Value,
    algorithm: KeyGenAlgorithm,
    extractable: bool,
    key_usages: Vec<js::JsString>,
) -> Result<Native<CryptoKey>> {
    if fmt.as_str() != "raw" {
        bail!("unsupported import format: {fmt}");
    }
    use js::FromJsValue;
    let key_data = js::Bytes::from_js_value(key_data)?;
    let key = CryptoKey {
        r#type: "secret".into(),
        extractable,
        algorithm,
        usages: key_usages,
        raw: key_data.as_bytes().to_vec(),
    };
    Native::new(&ctx, key)
}

#[js::host_call]
fn export_key(fmt: js::JsString, key: Native<CryptoKey>) -> Result<js::Bytes> {
    let key = key.borrow();
    if !key.extractable {
        bail!("key is not extractable");
    }
    match fmt.as_str() {
        "raw" => Ok(key.raw.clone().into()),
        _ => bail!("unsupported export format: {fmt}"),
    }
}

#[js::host_call]
fn get_random_values(output: js::JsUint8Array) -> Result<js::JsUint8Array> {
    let mut buf = alloc::vec![0u8; output.len()];
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
    let crypto = g.context()?.new_object("Crypto");
    let subtle = g.context()?.new_object("SubtleCrypto");
    setup_subtle(&subtle)?;
    crypto.set_property("subtle", &subtle)?;
    crypto.define_property_fn("getRandomValues", get_random_values)?;
    crypto.define_property_fn("randomUUID", random_uuid)?;
    g.set_property("crypto", &crypto)?;
    Ok(())
}
