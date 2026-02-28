use falcon_core::{keygen, sign};

fn main() {
    println!("🚀 Falcon CLI - Testiranje potpisa");
    println!("----------------------------------");

    // 1. Generisanje ključeva
    println!("⏳ Generišem Falcon-512 ključeve...");
    let (pk, sk) = match keygen::generate_keypair() {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("❌ Greška: {}", e);
            return;
        }
    };
    println!("✅ Ključevi spremni!");

    // 2. Poruka koju potpisujemo
    let poruka = b"Ovo je jako bitna poruka za master rad - batch agregacija!";
    println!("📄 Poruka: \"{}\"", String::from_utf8_lossy(poruka));

    // 3. Potpisivanje
    println!("⏳ Potpisujem poruku...");
    let potpis = match sign::sign_message(poruka, &sk) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Greška pri potpisivanju: {}", e);
            return;
        }
    };
    println!("✅ Poruka potpisana! Dužina potpisa: {} bajtova", potpis.len());

    // 4. Verifikacija
    println!("⏳ Verifikujem potpis...");
    let is_valid = sign::verify_signature(poruka, &potpis, &pk);

    if is_valid {
        println!("🎉 USPJEH: Potpis je VALIDAN!");
    } else {
        println!("🚨 GREŠKA: Potpis NIJE validan!");
    }
}