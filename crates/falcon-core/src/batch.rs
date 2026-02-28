use crate::bindings;

/// Batch verifikacija za isti javni ključ (Scenarij A)
pub fn verify_batch_same_key(pk: &[u8], msgs: &[&[u8]], sigs: &[&[u8]]) -> bool {
    // Osnovna provjera: broj poruka mora biti jednak broju potpisa
    if msgs.len() != sigs.len() || msgs.is_empty() {
        return false;
    }
    
    let num_sigs = msgs.len();

    // Dodano 'mut' na vektore pokazivača
    let mut msg_ptrs: Vec<*const u8> = msgs.iter().map(|m| m.as_ptr()).collect();
    let msg_lens: Vec<usize> = msgs.iter().map(|m| m.len()).collect();

    let mut sig_ptrs: Vec<*const u8> = sigs.iter().map(|s| s.as_ptr()).collect();
    let sig_lens: Vec<usize> = sigs.iter().map(|s| s.len()).collect();

    let result = unsafe {
        bindings::falcon512_batch_verify_same_key(
            pk.as_ptr(),
            msg_ptrs.as_mut_ptr(), // Koristimo as_mut_ptr() za pokazivače
            msg_lens.as_ptr(),     // Dužine ostaju as_ptr() jer su proslijeđene kao const *size_t
            sig_ptrs.as_mut_ptr(),
            sig_lens.as_ptr(),
            num_sigs,
        )
    };

    result == 0
}

/// Batch verifikacija za različite javne ključeve (Scenarij B)
pub fn verify_batch_diff_keys(pks: &[&[u8]], msgs: &[&[u8]], sigs: &[&[u8]]) -> bool {
    // Broj ključeva, poruka i potpisa mora biti identičan
    if pks.len() != msgs.len() || msgs.len() != sigs.len() || msgs.is_empty() {
        return false;
    }
    
    let num_sigs = msgs.len();

    // Dodano 'mut'
    let mut pk_ptrs: Vec<*const u8> = pks.iter().map(|k| k.as_ptr()).collect();
    let mut msg_ptrs: Vec<*const u8> = msgs.iter().map(|m| m.as_ptr()).collect();
    let msg_lens: Vec<usize> = msgs.iter().map(|m| m.len()).collect();
    let mut sig_ptrs: Vec<*const u8> = sigs.iter().map(|s| s.as_ptr()).collect();
    let sig_lens: Vec<usize> = sigs.iter().map(|s| s.len()).collect();

    let result = unsafe {
        bindings::falcon512_batch_verify_diff_keys(
            pk_ptrs.as_mut_ptr(),
            msg_ptrs.as_mut_ptr(),
            msg_lens.as_ptr(),
            sig_ptrs.as_mut_ptr(),
            sig_lens.as_ptr(),
            num_sigs,
        )
    };

    result == 0
}