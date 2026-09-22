//! Polyvocoder Rust port
//!
//! The Rust port provides the same FNV-1a 64-bit hash and stable
//! canonical features as the Python and TypeScript implementations.
//!
//! This is the polyformalism port 7 of 7 (Python, TS, Rust, Bash, JS, C#, +Rust).

use serde::{Deserialize, Serialize};

/// FNV-1a 64-bit offset basis
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
/// FNV-1a 64-bit prime
const FNV_PRIME: u64 = 0x100000001b3;

/// The Quilt fleet canary string.
///
/// Encoding this as UTF-8 and hashing via FNV-1a must yield
/// 0x024a555471370b18d (or equivalently 0x24a555471370b18d when leading
/// zeros are stripped — same number, = 2,640,610,520,279,855,501).
pub const CANARY_STRING: &str = "café Δ 日本語";

/// Compute FNV-1a 64-bit hash of a string slice (UTF-8 encoded).
pub fn fnv1a_64(s: &str) -> u64 {
    let mut h = FNV_OFFSET_BASIS;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

/// Compute the Quilt fleet canary hash.
pub fn canary_hash() -> u64 {
    fnv1a_64(CANARY_STRING)
}

/// Verify the canary.
pub fn verify_canary() -> bool {
    canary_hash() == 0x024a555471370b18d
}

/// Generate dials (8 signed int16 values) from a numeric seed.
///
/// Mirrors the Python implementation. Hashes the seed (u64 LE), then takes
/// the first 8 bytes as signed int16, then mirrors to 16 with zeros.
pub fn generate_dials(seed: u64) -> [i16; 16] {
    let mut hasher = FnvHasher::new();
    hasher.hash_u64(seed);
    let hash = hasher.finish();

    // Take first 8 bytes
    let bytes = hash.to_le_bytes();
    let mut dials = [0i16; 16];
    for i in 0..4 {
        let lo = bytes[i * 2] as i16;
        let hi = bytes[i * 2 + 1] as i16;
        let val = (lo as i16) | ((hi as i16) << 8);
        dials[i] = val;
        dials[i + 4] = val; // mirror to positions 4-7
    }
    // positions 8-15 stay zero
    dials
}

/// Simple FNV-1a 64 hasher (for use with std::hash::Hasher trait).
#[derive(Default)]
pub struct FnvHasher {
    state: u64,
}

impl FnvHasher {
    pub fn new() -> Self {
        Self { state: FNV_OFFSET_BASIS }
    }

    pub fn hash_u64(&mut self, n: u64) {
        for b in n.to_le_bytes() {
            self.state ^= b as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }

    pub fn finish(&self) -> u64 {
        self.state
    }
}

/// JEVFeatures — schema parity with Python and TypeScript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JEVFeatures {
    #[serde(alias = "canon_worthy", alias = "canonWorthy")]
    pub canon_worthy: f64,
    #[serde(alias = "distinct_voice", alias = "distinctVoice")]
    pub distinct_voice: f64,
    #[serde(alias = "doctrine_anchor", alias = "doctrineAnchor")]
    pub doctrine_anchor: f64,
    #[serde(alias = "voice_fit", alias = "voiceFit")]
    pub voice_fit: f64,
    #[serde(alias = "novelty")]
    pub novelty: f64,
    #[serde(alias = "density")]
    pub density: f64,
}

/// DecodedSample — schema parity with Python.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedSample {
    #[serde(alias = "latent")]
    pub latent: Vec<f64>,
    #[serde(alias = "text")]
    pub text: String,
    #[serde(alias = "image")]
    pub image: String,
    #[serde(alias = "audio")]
    pub audio: Vec<f32>,
}

/// PipelineResult — schema parity with Python.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    #[serde(alias = "features")]
    pub features: JEVFeatures,
    #[serde(alias = "decoded_samples", alias = "decodedSamples")]
    pub decoded_samples: Vec<DecodedSample>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canary() {
        let h = canary_hash();
        assert_eq!(h, 0x024a555471370b18d);
    }

    #[test]
    fn test_dials() {
        let dials = generate_dials(42);
        assert_eq!(dials.len(), 16);
        // positions 8-15 are zeros
        for i in 8..16 {
            assert_eq!(dials[i], 0);
        }
    }

    #[test]
    fn test_fnv1a_empty() {
        // Empty string → offset basis
        assert_eq!(fnv1a_64(""), FNV_OFFSET_BASIS);
    }

    #[test]
    fn test_fnv1a_one_byte() {
        // Single byte 'a' (0x61)
        let h = fnv1a_64("a");
        assert_eq!(h, 0xaf63dc4c8601ec8c);
    }

    #[test]
    fn test_features_serde() {
        let f = JEVFeatures {
            canon_worthy: 0.5,
            distinct_voice: 0.6,
            doctrine_anchor: 0.7,
            voice_fit: 0.8,
            novelty: 0.4,
            density: 1.0,
        };
        let json = serde_json::to_string(&f).unwrap();
        assert!(json.contains("canonWorthy"));
        assert!(json.contains("canon_worthy"));
    }
}
