use crate::{H256, traits::Hasher};
use core::convert::TryInto;
use sha2::{Digest, Sha256};

const SHA256_DIGEST_BYTE_LEN: usize = core::mem::size_of::<H256>();
const SHA256_PERSONALIZATION_STRING: &[u8] = b"sparsemerkletree";

pub struct Sha256Hasher(Sha256);

impl Default for Sha256Hasher {
    fn default() -> Self {
        let mut hasher = Sha256::new();
        hasher.update(SHA256_PERSONALIZATION_STRING);

        Self(hasher)
    }
}

impl Hasher for Sha256Hasher {
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    fn finish(self) -> H256 {
        let hash = self.0.finalize();
        let bytes: [u8; SHA256_DIGEST_BYTE_LEN] = hash.as_slice().try_into().expect("Sha256 output conversion to fixed array shouldn't fail");

        bytes.into()
    }
}
