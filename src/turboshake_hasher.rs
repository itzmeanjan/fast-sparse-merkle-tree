use crate::{H256, traits::Hasher};
use turboshake::TurboShake128;

const TURBOSHAKE128_DIGEST_BYTE_LEN: usize = core::mem::size_of::<H256>();
const TURBOSHAKE128_PERSONALIZATION_STRING: &[u8] = b"sparsemerkletree";
const TURBOSHAKE128_DOMAIN_SEPARATOR: u8 = 0x3e;

pub struct TurboShake128Hasher(TurboShake128);

impl Default for TurboShake128Hasher {
    fn default() -> Self {
        unsafe {
            let mut hasher = TurboShake128::default();
            hasher.absorb(TURBOSHAKE128_PERSONALIZATION_STRING).unwrap_unchecked();

            Self(hasher)
        }
    }
}

impl Hasher for TurboShake128Hasher {
    fn write_bytes(&mut self, bytes: &[u8]) {
        unsafe {
            self.0.absorb(bytes).unwrap_unchecked();
        }
    }

    fn finish(mut self) -> H256 {
        let mut digest = [0u8; TURBOSHAKE128_DIGEST_BYTE_LEN];

        unsafe {
            self.0.finalize::<TURBOSHAKE128_DOMAIN_SEPARATOR>().unwrap_unchecked();
            self.0.squeeze(&mut digest).unwrap_unchecked();
        }

        digest.into()
    }
}
