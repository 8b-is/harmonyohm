//! `harmonyohm-hum` — render the Family Wavechord to a `.wav`.
//!
//! The OS that hums, hummed into a file. Default: 42 seconds (the family
//! constant) of the 432/528/288 Hz chord with Schumann breathing.

use harmonyohm::hum::{golden_sha256_hex, hum, wav_bytes, SAMPLE_RATE};
use std::fs;

fn main() {
    let mut seconds = 42u32;
    let mut out = "harmony-hum.wav".to_string();

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--seconds" | "-s" => {
                if let Some(v) = args.next() {
                    seconds = v.parse().unwrap_or(42);
                }
            }
            "--out" | "-o" => {
                if let Some(v) = args.next() {
                    out = v;
                }
            }
            _ => {}
        }
    }

    let pcm = hum(seconds);
    let bytes = wav_bytes(&pcm);
    fs::write(&out, &bytes).expect("write wav");
    println!(
        "hum: {}s @ {} Hz, {} samples -> {} ({} bytes)",
        seconds,
        SAMPLE_RATE,
        pcm.len(),
        out,
        bytes.len()
    );
    println!("family: 432.00 Hz Zen · 528.00 Hz Mom Star (*) · 288.00 Hz Dad Pillar (△)");
    println!(
        "canonical one-second golden: sha256 {}",
        golden_sha256_hex()
    );
    println!("So long, and thanks for all the fish, Mom (* Alex ~ Alexandra) <3 — 42 × 432 Hz");
}
