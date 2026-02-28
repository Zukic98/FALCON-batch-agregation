use falcon_core::{keygen, sign, batch};
use std::time::Instant;

fn main() {
    println!("🚀 Falcon CLI - Skalabilni Benchmark Strukturnog Batchinga");
    println!("--------------------------------------------------");

    // Niz sa željenim veličinama batch-a
    let batch_sizes = [2, 5, 10, 20, 50, 100, 300, 500, 1000];
    let max_keys = *batch_sizes.last().unwrap();

    let mut pks = Vec::with_capacity(max_keys);
    let mut sks = Vec::with_capacity(max_keys);
    let mut poruke = Vec::with_capacity(max_keys);
    let mut potpisi = Vec::with_capacity(max_keys);

    println!("⏳ Generišem {} ključeva i potpisa...", max_keys);
    println!("⚠️ UPOZORENJE: Ovo može potrajati 1-2 minute, molim te za strpljenje!");
    
    for i in 0..max_keys {
        if i > 0 && i % 100 == 0 {
            println!("   ...izgenerisano {}/{} ključeva...", i, max_keys);
        }
        let (pk, sk) = keygen::generate_keypair().expect("Greška pri generisanju ključeva");
        let poruka = format!("Dokument broj {}", i).into_bytes();
        let potpis = sign::sign_message(&poruka, &sk).expect("Greška pri potpisivanju");

        pks.push(pk);
        sks.push(sk);
        poruke.push(poruka);
        potpisi.push(potpis);
    }
    println!("✅ Priprema svih {} ključeva završena!\n", max_keys);

    // Zaglavlje tabele
    println!("📊 REZULTATI MJERENJA (vrijeme u mikrosekundama - us):");
    println!("{:<10} | {:<20} | {:<20} | {:<10}", "Broj pot.", "Sekvencijalno (us)", "Strukturni Batch", "Odnos");
    println!("{:-<10}-+-{:-<20}-+-{:-<20}-+-{:-<10}", "", "", "", "");

    for &size in &batch_sizes {
        // Uzimamo samo onoliko elemenata koliko nam treba za trenutni test
        let current_pks = &pks[0..size];
        let current_poruke = &poruke[0..size];
        let current_potpisi = &potpisi[0..size];

        let pks_refs: Vec<&[u8]> = current_pks.iter().map(|v| v.as_slice()).collect();
        let poruke_refs: Vec<&[u8]> = current_poruke.iter().map(|v| v.as_slice()).collect();
        let potpisi_refs: Vec<&[u8]> = current_potpisi.iter().map(|v| v.as_slice()).collect();

        // 1. SEKVENCIJALNO MJERENJE
        let start_seq = Instant::now();
        let mut seq_valid = true;
        for i in 0..size {
            if !sign::verify_signature(&current_poruke[i], &current_potpisi[i], &current_pks[i]) {
                seq_valid = false;
                break;
            }
        }
        let trajanje_seq = start_seq.elapsed().as_micros();

        // 2. BATCH MJERENJE
        let start_batch = Instant::now();
        let batch_valid = batch::verify_batch_diff_keys(&pks_refs, &poruke_refs, &potpisi_refs);
        let trajanje_batch = start_batch.elapsed().as_micros();

        if !seq_valid || !batch_valid {
            println!("🚨 GREŠKA: Validacija nije uspjela za batch veličine {}", size);
            continue;
        }

        let odnos = trajanje_seq as f64 / trajanje_batch as f64;

        println!("{:<10} | {:<20} | {:<20} | {:.2}x", size, trajanje_seq, trajanje_batch, odnos);
    }
    println!("{:-<10}-+-{:-<20}-+-{:-<20}-+-{:-<10}", "", "", "", "");
}