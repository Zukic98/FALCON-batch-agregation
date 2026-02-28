#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Povezujemo naš novi modul za ključeve
pub mod keygen;
pub mod sign;
pub mod batch;