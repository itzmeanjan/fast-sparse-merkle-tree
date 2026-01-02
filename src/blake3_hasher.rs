use crate::{H256, traits::Hasher};

const BLAKE3_DIGEST_BYTE_LEN: usize = core::mem::size_of::<H256>();
const BLAKE3_PERSONALIZATION_STRING: &[u8] = b"sparsemerkletree";

pub struct Blake3Hasher(blake3::Hasher);

impl Default for Blake3Hasher {
    fn default() -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(BLAKE3_PERSONALIZATION_STRING);

        Self(hasher)
    }
}

impl Hasher for Blake3Hasher {
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    fn finish(self) -> H256 {
        let mut digest = [0u8; BLAKE3_DIGEST_BYTE_LEN];
        digest.copy_from_slice(self.0.finalize().as_bytes());

        digest.into()
    }
}
