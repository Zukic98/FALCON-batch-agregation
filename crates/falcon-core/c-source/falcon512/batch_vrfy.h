#ifndef BATCH_VRFY_H
#define BATCH_VRFY_H

#include <stddef.h>
#include <stdint.h>

/*
 * SCENARIJ A: Batch verifikacija više poruka sa ISTIM javnim ključem.
 * Vraća 0 ako su svi potpisi validni, ili negativan broj na prvom nevalidnom.
 */
int falcon512_batch_verify_same_key(
    const uint8_t *pk,
    const uint8_t **msgs, const size_t *msg_lens,
    const uint8_t **sigs, const size_t *sig_lens,
    size_t num_sigs
);

/*
 * SCENARIJ B: Batch verifikacija više poruka sa RAZLIČITIM javnim ključevima.
 * Vraća 0 ako su svi potpisi validni, ili negativan broj na prvom nevalidnom.
 */
int falcon512_batch_verify_diff_keys(
    const uint8_t **pks,
    const uint8_t **msgs, const size_t *msg_lens,
    const uint8_t **sigs, const size_t *sig_lens,
    size_t num_sigs
);

#endif