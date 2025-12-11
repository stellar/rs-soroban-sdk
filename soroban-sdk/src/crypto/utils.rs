use crate::BytesN;

// This routine was copied with slight modification from the arkworks library:
// https://github.com/arkworks-rs/algebra/blob/bf1c9b22b30325ef4df4f701dedcb6dea904c587/ff/src/biginteger/arithmetic.rs#L66-L79
fn sbb_for_sub_with_borrow(a: &mut u64, b: u64, borrow: u8) -> u8 {
    let tmp = (1u128 << 64) + u128::from(*a) - u128::from(b) - u128::from(borrow);
    // casting is safe here because `tmp` can only exceed u64 by a single
    // (borrow) bit, which we capture in the next line.
    *a = tmp as u64;
    u8::from(tmp >> 64 == 0)
}

#[derive(Debug)]
pub(crate) struct BigInt<const N: usize>(pub [u64; N]);

impl<const N: usize> BigInt<N> {
    pub fn sub_with_borrow(&mut self, other: &Self) -> bool {
        let mut borrow = 0;
        for i in 0..N {
            borrow = sbb_for_sub_with_borrow(&mut self.0[i], other.0[i], borrow);
        }
        borrow != 0
    }

    pub fn copy_into_array<const M: usize>(&self, slice: &mut [u8; M]) {
        const {
            if M != N * 8 {
                panic!("BigInt::copy_into_array with mismatched array length")
            }
        }

        for i in 0..N {
            let limb_bytes = self.0[N - 1 - i].to_be_bytes();
            slice[i * 8..(i + 1) * 8].copy_from_slice(&limb_bytes);
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0; N]
    }
}

impl<const N: usize, const M: usize> From<&BytesN<M>> for BigInt<N> {
    fn from(bytes: &BytesN<M>) -> Self {
        if M != N * 8 {
            panic!("BytesN::Into<BigInt> - length mismatch")
        }

        let array = bytes.to_array();
        let mut limbs = [0u64; N];
        for i in 0..N {
            let start = i * 8;
            let end = start + 8;
            let mut chunk = [0u8; 8];
            chunk.copy_from_slice(&array[start..end]);
            limbs[N - 1 - i] = u64::from_be_bytes(chunk);
        }
        BigInt(limbs)
    }
}
