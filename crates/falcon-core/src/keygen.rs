use crate::bindings;

// Dohvaćamo tačne veličine ključeva iz C header fajlova
pub const PUBLIC_KEY_BYTES: usize = bindings::PQCLEAN_FALCON512_CLEAN_CRYPTO_PUBLICKEYBYTES as usize;
pub const SECRET_KEY_BYTES: usize = bindings::PQCLEAN_FALCON512_CLEAN_CRYPTO_SECRETKEYBYTES as usize;

/// Generiše Falcon-512 par ključeva (Javni i Tajni)
pub fn generate_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    // Pripremamo prazne nizove (buffere) koje će C kod popuniti
    let mut pk = vec![0u8; PUBLIC_KEY_BYTES];
    let mut sk = vec![0u8; SECRET_KEY_BYTES];

    // Pozivamo nebezbjednu (unsafe) C funkciju
    let result = unsafe {
        bindings::PQCLEAN_FALCON512_CLEAN_crypto_sign_keypair(
            pk.as_mut_ptr(),
            sk.as_mut_ptr(),
        )
    };

    // C funkcije obično vraćaju 0 kada je sve u redu
    if result == 0 {
        Ok((pk, sk))
    } else {
        Err("C kod nije uspio generisati ključeve.")
    }
}