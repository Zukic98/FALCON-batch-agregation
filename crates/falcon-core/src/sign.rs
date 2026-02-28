use crate::bindings;

// Maksimalna dužina potpisa za Falcon-512
pub const SIGNATURE_BYTES: usize = bindings::PQCLEAN_FALCON512_CLEAN_CRYPTO_BYTES as usize;

/// Potpisuje poruku koristeći tajni ključ
pub fn sign_message(msg: &[u8], sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut sig = vec![0u8; SIGNATURE_BYTES];
    let mut siglen: usize = 0;

    let result = unsafe {
        bindings::PQCLEAN_FALCON512_CLEAN_crypto_sign_signature(
            sig.as_mut_ptr(),
            &mut siglen,
            msg.as_ptr(),
            msg.len(),
            sk.as_ptr(),
        )
    };

    if result == 0 {
        // C kod nam vraća stvarnu dužinu potpisa u 'siglen' (obično oko 666 bajtova)
        // Zato skraćujemo vektor na tu tačnu dužinu
        sig.truncate(siglen);
        Ok(sig)
    } else {
        Err("C kod nije uspio potpisati poruku.")
    }
}

/// Verifikuje potpis koristeći javni ključ
pub fn verify_signature(msg: &[u8], sig: &[u8], pk: &[u8]) -> bool {
    let result = unsafe {
        bindings::PQCLEAN_FALCON512_CLEAN_crypto_sign_verify(
            sig.as_ptr(),
            sig.len(),
            msg.as_ptr(),
            msg.len(),
            pk.as_ptr(),
        )
    };
    
    // Ako C funkcija vrati 0, potpis je validan
    result == 0
}