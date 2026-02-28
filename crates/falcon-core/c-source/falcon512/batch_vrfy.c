#include "batch_vrfy.h"
#include "api.h" // Uključujemo glavni PQClean API za Falcon

int falcon512_batch_verify_same_key(
    const uint8_t *pk,
    const uint8_t **msgs, const size_t *msg_lens,
    const uint8_t **sigs, const size_t *sig_lens,
    size_t num_sigs
) {
    // Prolazimo kroz sve potpise u batch-u
    for (size_t i = 0; i < num_sigs; i++) {
        int result = PQCLEAN_FALCON512_CLEAN_crypto_sign_verify(
            sigs[i], sig_lens[i],
            msgs[i], msg_lens[i],
            pk
        );
        
        // Ako barem jedan potpis padne, cijeli batch je nevalidan
        if (result != 0) {
            return -1; 
        }
    }
    return 0; // Svi su validni
}

int falcon512_batch_verify_diff_keys(
    const uint8_t **pks,
    const uint8_t **msgs, const size_t *msg_lens,
    const uint8_t **sigs, const size_t *sig_lens,
    size_t num_sigs
) {
    for (size_t i = 0; i < num_sigs; i++) {
        int result = PQCLEAN_FALCON512_CLEAN_crypto_sign_verify(
            sigs[i], sig_lens[i],
            msgs[i], msg_lens[i],
            pks[i]
        );
        
        if (result != 0) {
            return -1;
        }
    }
    return 0;
}