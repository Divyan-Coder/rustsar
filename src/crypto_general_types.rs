/// # Module: crypto_general_types
///
/// This module defines standard types used in the AUTOSAR Classic Platform base software.
///
/// ## Description:
/// This file contains all type definitions of CSM modules define under crypto_general_types.
/// It adheres to the AUTOSAR specifications and provides compatibility with the specified version.
///
/// ## AUTOSAR Details:
/// - **AUTOSAR Version:** R21-11
/// - **Classic Platform:** Yes
///
/// ## Version History:
/// --------------------------------------------------------------------------------------------
/// | Version | Date       | Author         | Description                                      |
/// --------------------------------------------------------------------------------------------
/// | 0.0.1   | 2025-11-05 | Divyan-coder   | Initial version of std_types.rs file.            |
/// --------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CryptoAlgorithmFamilyType {
    CryptoAlgoFamNotSet = 0,
    CryptoAlgofamSha1           = 0x01 , //SHA1 hash
    CryptoAlgofamSha2_224     = 0x02 , //SHA2-224 hash
    CryptoAlgofamSha2_256       = 0x03 , // SHA2-256 hash
    CryptoAlgofamSha2_384       = 0x04 , //SHA2-384 hash
    CryptoAlgofamSha2_512       = 0x05 , //SHA2-512 hash
    CryptoAlgofamSha2_512_224   = 0x06 , //SHA2-512/224 hash
    CryptoAlgofamSha2_512_256   = 0x07 , //SHA2-512/256 hash
    CryptoAlgofamSha3_224       = 0x08 , //SHA3-224 hash
    CryptoAlgofamSha3_256       = 0x09 , //
    CryptoAlgofamSha3_384       = 0x0a , //SHA3-384 hash
    CryptoAlgofamSha3_512       = 0x0b , //SHA3-512 hash
    CryptoAlgofamShake128       = 0x0c , //SHAKE128 hash
    CryptoAlgofamShake256       = 0x0d , //SHAKE256 hash
    CryptoAlgofamRipemd160 = 0x0e , //RIPEMD hash
    CryptoAlgofamBlake1_256 = 0x0f , //BLAKE-1-256 hash
    CryptoAlgofamBlake1_512 = 0x10 , //BLAKE-1-512 hash
    CryptoAlgofamBlake2s256 = 0x11 , //BLAKE-2s-256 hash
    CryptoAlgofamBlake2s512 = 0x12 , //BLAKE-2s-512 hash
    CryptoAlgofam3des = 0x13 , //3DES cipher
    CryptoAlgofamAes = 0x14 , //AES cipher
    CryptoAlgofamChacha = 0x15 , //ChaCha cipher
    CryptoAlgofamRsa = 0x16 , //RSA cipher
    CryptoAlgofamEd25519 = 0x17 , //ED22519 elliptic curve
    CryptoAlgofamBrainpool = 0x18 , //Brainpool elliptic curve
    CryptoAlgofamEccnist = 0x19 , //NIST ECC elliptic curves
    CryptoAlgofamRng = 0x1b , //Random Number Generator
    CryptoAlgofamSiphash = 0x1c , //SipHash
    CryptoAlgofamEccansi = 0x1e , //Elliptic curve according to ANSI X9.62
    CryptoAlgofamEccsec = 0x1f , //Elliptic curve according to SECG
    CryptoAlgofamDrbg = 0x20 , //Random number generator according to NIST SP800-90A
    CryptoAlgofamFips186 = 0x21 , //Random number generator according to FIPS 186.
    CryptoAlgofamPaddingPkcs7 = 0x22 , //Cipher padding according to PKCS.7
    CryptoAlgofamPaddingOnewithzeros = 0x23 , //Cipher padding mode. Fill/verify data with 0, but first bit after the data is 1. Eg. "DATA" & 0x80 & 0x00...
    CryptoAlgofamPbkdf2 = 0x24 , //Password-Based Key Derivation Function 2
    CryptoAlgofamKdfx963 = 0x25 , //ANSI X9.63 Public Key Cryptography
    CryptoAlgofamDh = 0x26 , //Diffie-Hellman
    CryptoAlgofamSm2 = 0x27 , //SM2 elliptic curve algorithm
    CryptoAlgofamEea3 = 0x28 , //Stream cipher based on [x01]
    CryptoAlgofamSm3 = 0x29 , //Chinese hash algorithm based on [x02]
    CryptoAlgofamEia3 = 0x2A , //Authentication algorithm [x01]
    CryptoAlgofamHkdf = 0x2B , //HMAC-based extract-and-expand key derivation function
    CryptoAlgofamEcdsa = 0x2C , //Elliptic-curve Digital Signatures
    CryptoAlgofamPoly1305 = 0x2D , //MAC calculation algorithm
    CryptoAlgofamX25519 = 0x2E , //Elliptic curve X25519 for ECDH
    CryptoAlgofamEcdh = 0x2F  , //Elliptic-curve Diffie Hellman
    CryptoAlgoFamCustom = 0xff, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CryptoAlgorithmModeType {
    CryptoAlgoModeNotSet = 0,
    CryptoAlgomodeEcb = 0x01 , //Blockmode: Electronic Codebook
    CryptoAlgomodeCbc = 0x02  , //Blockmode: Cipher Block Chaining
    CryptoAlgomodeCfb = 0x03 , //Blockmode: Cipher Feedback Mode
    CryptoAlgomodeOfb = 0x04 , //Blockmode: Output Feedback Mode
    CryptoAlgomodeCtr = 0x05 , //Blockmode: Counter Modex
    CryptoAlgomodeGcm = 0x06 , //Blockmode: Galois/Counter Mode
    CryptoAlgomodeXts = 0x07 , //XEX Tweakable Block Cipher with Ciphertext Stealing
    CryptoAlgomodeRsaesOaep = 0x08 , //RSA Optimal Asymmetric Encryption Padding
    CryptoAlgomodeRsaesPkcs1V1_5 = 0x09 , //RSA encryption/decryption with PKCS#1 v1.5 padding
    CryptoAlgomodeRsassaPss = 0x0a , //RSA Probabilistic Signature Scheme
    CryptoAlgomodeRsassaPkcs1V1_5 = 0x0b , //RSA signature with PKCS#1 v1.5
    CryptoAlgomode8rounds = 0x0c , //8 rounds (e.g. ChaCha8)
    CryptoAlgomode12rounds = 0x0d , //12 rounds (e.g. ChaCha12)
    CryptoAlgomode20rounds = 0x0e , //20 rounds (e.g. ChaCha20)
    CryptoAlgomodeHmac = 0x0f , //Hashed-based MAC
    CryptoAlgomodeCmac = 0x10 , //Cipher-based MAC
    CryptoAlgomodeGmac = 0x11 , //Galois MAC
    CryptoAlgomodeCtrdrbg = 0x12 , //Counter-based Deterministic Random Bit Generator
    CryptoAlgomodeSiphash2_4 = 0x13 , //Siphash-2-4
    CryptoAlgomodeSiphash4_8 = 0x14 , //Siphash-4-8
    CryptoAlgomodePxxxr1 = 0x15 , //ANSI R1 Curve
    CryptoAlgoModeCustom = 0xFF,
}

/// Input/Output redirection configuration (maps input/output buffers to
/// different logical channels)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CryptoInputOutputRedirectionConfigType {
    PrimaryInput = 0,
    SecondaryInput = 1,
    TertiaryInput = 2,
    PrimaryOutput = 3,
    SecondaryOutput = 4,
}

/// Job state for the Crypto Service Manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CryptoJobStateType {
    Idle = 0,
    Active = 1,
}

/// Result of a verification operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CryptoVerifyResultType {
    Unknown = 0,
    Verified = 1,
    NotVerified = 2,
}

pub enum CryptoServiceInfoType{
    Hash = 0,
    MacGenerate = 1,
    CryptoMacverify = 0x02 , //MacVerify Service
    CryptoEncrypt = 0x03 , //Encrypt Service
    CryptoDecrypt = 0x04 , //Decrypt Service
    CryptoAeadencrypt = 0x05 , //AEADEncrypt Service
    CryptoAeaddecrypt = 0x06 , //AEADDecrypt Service
    CryptoSignaturegenerate = 0x07 , //SignatureGenerate Service
    CryptoSignatureverify = 0x08 , //SignatureVerify Service
    CryptoRandomgenerate = 0x0B , //RandomGenerate Service
    CryptoRandomseed = 0x0C , //RandomSeed Service
    CryptoKeygenerate = 0x0D , //KeyGenerate Service
    CryptoKeyderive = 0x0E , //KeyDerive Service
    CryptoKeyexchangecalcpubval = 0x0F , //KeyExchangeCalcPubVal Service
    CryptoKeyexchangecalcsecret = 0x10 , //KeyExchangeCalcSecret Service
    CryptoKeysetvalid = 0x13 , //KeySetValid Service 
    CryptoKeysetinvalid = 0x14 , //KeySetInvalid Service
    CryptoCustomService = 0x15 , //Custom service job
}

/// Status of a key stored in the CryptoIf/key manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CryptoKeyStatusType {
    CryptoKeyStatusInvalid = 0,
    CryptoKeyStatusValid = 1,
}

/// Generic result type used for notifications/callbacks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CryptoResultType {
    CryptoResultOk = 0,
    CryptoResultFailed = 1,
}

#[derive(Debug, Clone)]
pub enum CryptoProcessingType {
    Sync = 0,
    Async = 1,
}

pub enum CryptoOperationModeType {
    OperationStart =1,
    OperationUpdate =2,
    OperationStreamstart =3,
    OperationmodeFinish =4,
    OperationmodeSinglecall =7,
    OperationmodeSaveContext =8,
    OperationmodeRestoreContext =9,
}
pub struct CryptoPrimitiveInfoType{
    service : CryptoServiceInfoType,
    algorithm : CryptoAlgorithmInfoType,
}

/// Primitive information for a job (algorithm identifiers and parameters).
/// Fields here are intentionally generic; concrete SRS-driven fields can be
/// extended later.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CryptoJobPrimitiveInfoType {
    callback_id: u32,
    primitive_info : Option<*mut CryptoPrimitiveInfoType>,
    cry_if_key_id: u32,
    processing_type : CryptoProcessingType,
}

/// Redirection descriptor used to route inputs/outputs to different buffers
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CryptoJobRedirectionInfoType {
    pub redirect_config: u8,
    input_key_id : u32,
    input_key_element_id : u32,
    secondary_input_key_id : u32,
    secondary_input_key_element_id : u32,
    tertiary_input_key_id : u32,
    tertiary_input_key_element_id : u32,
    output_key_id : u32,
    output_key_element_id : u32,
    secondary_output_key_id : u32,
    secondary_output_key_element_id : u32,
}

/// Input/output buffer set for a job primitive. Uses raw pointers to match the
/// header semantics; higher-level wrappers should expose safe slice-based
/// APIs.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CryptoJobPrimitiveInputOutputType {
    pub input_ptr: Option<*const u8>,
    pub input_length: u32,
    pub secondary_input_ptr: Option<*const u8>,
    pub secondary_input_length: u32,
    pub tertiary_input_ptr: Option<*const u8>,
    pub tertiary_input_length: u32,
    pub output_ptr: Option<*mut u8>,
    /// pointer to a u32 where the driver writes the actual output length
    pub output_length_ptr: Option<*mut u32>,
    pub secondary_output_ptr: Option<*mut u8>,
    pub secondary_output_length_ptr: Option<*mut u32>,
    pub verify_ptr: Option<*mut CryptoVerifyResultType>,
    mode : Option<*mut CryptoOperationModeType>,
    cry_if_key_id: u32,
    target_cry_if_key_id: u32,
}

/// Top-level job descriptor used by the Crypto Service Manager
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CryptoJobType {
    pub job_id: u32,
    pub job_state: CryptoJobStateType,
    pub job_primitive_input_output: CryptoJobPrimitiveInputOutputType,
    pub job_primitive_info: CryptoJobPrimitiveInfoType,
    /// Optional reference to redirection information (may be None)
    pub job_redirection_info_ref: Option<*const CryptoJobRedirectionInfoType>,
    pub crypto_key_id: u32,
    pub target_crypto_key_id: u32,
    pub job_priority: u32,
}

pub struct CryptoAlgorithmInfoType{
    family: CryptoAlgorithmFamilyType,
    secondary_family: CryptoAlgorithmFamilyType,
    key_length: u32,
    mode : CryptoAlgorithmModeType,
}
