//! Const SHA-256, kept in its own module so that it can be fuzzed.
//!
//! This module must stay free of `crate::` references and of anything beyond
//! `core`, because `soroban-spec/fuzz` includes this file directly by path in
//! order to differential-fuzz it against the `sha2` crate without `sha256`
//! becoming part of the crate's public API. Adding a dependency on the rest of
//! the crate here would break that fuzz target.

/// SHA-256 round constants.
#[rustfmt::skip]
const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA-256 of `input`, evaluatable at compile time.
///
/// A const implementation is needed because the marker is built while
/// macro-generated contract code compiles, and `sha2` is not usable in a const
/// context: the RustCrypto `Digest` trait family can't offer `const fn`
/// methods. See <https://github.com/RustCrypto/hashes/issues/288>.
pub(crate) const fn sha256(input: &[u8]) -> [u8; 32] {
    let len = input.len();
    // Message is padded with a 0x80 byte, then zeroes, then the length in bits
    // as a big-endian u64, out to a multiple of the 64-byte block size.
    let padded = (len + 9).div_ceil(64) * 64;
    let bit_len = (len as u64) * 8;

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let mut base = 0;
    while base < padded {
        let mut w = [0u32; 64];

        // Load the block, reading padding bytes rather than materialising a
        // padded copy of the message.
        let mut t = 0;
        while t < 16 {
            let mut word = 0u32;
            let mut b = 0;
            while b < 4 {
                let p = base + t * 4 + b;
                let byte = if p < len {
                    input[p]
                } else if p == len {
                    0x80
                } else if p >= padded - 8 {
                    (bit_len >> (8 * (padded - 1 - p))) as u8
                } else {
                    0
                };
                word = (word << 8) | (byte as u32);
                b += 1;
            }
            w[t] = word;
            t += 1;
        }

        let mut t = 16;
        while t < 64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
            t += 1;
        }

        let (mut a, mut b, mut c, mut d) = (h[0], h[1], h[2], h[3]);
        let (mut e, mut f, mut g, mut hh) = (h[4], h[5], h[6], h[7]);

        let mut t = 0;
        while t < 64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[t])
                .wrapping_add(w[t]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
            t += 1;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);

        base += 64;
    }

    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 8 {
        let be = h[i].to_be_bytes();
        out[i * 4] = be[0];
        out[i * 4 + 1] = be[1];
        out[i * 4 + 2] = be[2];
        out[i * 4 + 3] = be[3];
        i += 1;
    }
    out
}
