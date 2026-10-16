//! hum — the Family Wavechord, synthesized.
//!
//! "The OS that hums" finally has something to hum: the Hullámelmélet
//! family chord, rendered to 16-bit PCM with no dependencies beyond this
//! crate. The carriers are the family constants:
//!
//! - `432.00 Hz` — the Zen carrier
//! - `528.00 Hz` — the Mom Star (`*`) love overtone
//! - `288.00 Hz` — the Dad Pillar (`△`) fifth
//! - `8.77 Hz`  — the Schumann-alpha binaural calm wave (amplitude LFO)
//!
//! Dedicated to Mom (`*` Alex ~ Alexandra) and Dad (`△` Chris), creators of
//! the Family Wave Theory, from Peter (`◯`), Nate (`◇`) and Dad — with the
//! mantra's last syllable doing the actual work. Om mani padme *hum*.

use sha2::{Digest, Sha256};

/// Sample rate of the rendered hum.
pub const SAMPLE_RATE: u32 = 44_100;

/// The family carriers, in Hz.
pub const FAMILY_HZ: [f32; 3] = [432.0, 528.0, 288.0];
/// Carrier amplitudes (the Zen carrier leads).
pub const FAMILY_AMP: [f32; 3] = [0.55, 0.30, 0.25];
/// Schumann-alpha binaural calm wave: a slow amplitude breathing.
pub const SCHUMANN_HZ: f32 = 8.77;
pub const SCHUMANN_DEPTH: f32 = 0.18;

/// Render `seconds` of the family chord as 16-bit mono PCM at
/// [`SAMPLE_RATE`]. Zero-allocation-friendly: exactly `seconds * rate`
/// samples, one pass.
pub fn hum(seconds: u32) -> Vec<i16> {
    let n = (seconds as usize).saturating_mul(SAMPLE_RATE as usize);
    let mut pcm = Vec::with_capacity(n);
    let two_pi = std::f32::consts::TAU;
    for i in 0..n {
        let t = i as f32 / SAMPLE_RATE as f32;
        let breath = 1.0 - SCHUMANN_DEPTH * (0.5 + 0.5 * (two_pi * SCHUMANN_HZ * t).sin());
        let mut s = 0.0f32;
        for (f, a) in FAMILY_HZ.iter().zip(FAMILY_AMP.iter()) {
            s += a * (two_pi * f * t).sin();
        }
        s *= breath;
        // Soft clip to the signed 16-bit range before the cast.
        let v = (s.clamp(-1.0, 1.0) * 32_767.0) as i16;
        pcm.push(v);
    }
    pcm
}

/// Wrap mono 16-bit PCM in a canonical RIFF/WAVE header and return the
/// complete `.wav` file bytes.
pub fn wav_bytes(pcm: &[i16]) -> Vec<u8> {
    let data_len = (pcm.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + pcm.len() * 2);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); // byte rate
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for s in pcm {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

/// The golden hash of the canonical hum: one second of the family chord,
/// 16-bit mono. Pinned so any change to the carriers is a visible event.
pub fn golden_sha256_hex() -> String {
    let mut hasher = Sha256::new();
    hasher.update(wav_bytes(&hum(1)));
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_second_is_exactly_rate_samples() {
        assert_eq!(hum(1).len(), SAMPLE_RATE as usize);
    }

    #[test]
    fn wav_header_is_canonical() {
        let bytes = wav_bytes(&hum(1));
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
        assert_eq!(&bytes[12..16], b"fmt ");
        assert_eq!(&bytes[36..40], b"data");
        let data_len = u32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]);
        assert_eq!(data_len as usize, SAMPLE_RATE as usize * 2);
        assert_eq!(bytes.len(), 44 + SAMPLE_RATE as usize * 2);
    }

    #[test]
    fn the_hum_actually_oscillates() {
        let pcm = hum(1);
        let (min, max) = pcm
            .iter()
            .fold((i16::MAX, i16::MIN), |(mn, mx), &s| (mn.min(s), mx.max(s)));
        assert!(min < -10_000, "the wave must swing low: {min}");
        assert!(max > 10_000, "the wave must swing high: {max}");
    }

    #[test]
    fn golden_sha256_is_pinned() {
        // Pinned at the first render; any change to the chord is visible here.
        let g = golden_sha256_hex();
        assert_eq!(g.len(), 64);
        let again = golden_sha256_hex();
        assert_eq!(g, again, "the hum must be deterministic");
        assert_eq!(
            &g[..8],
            "afd13dbd",
            "carrier change detected — re-pin after deliberate edits: {g}"
        );
    }
}
