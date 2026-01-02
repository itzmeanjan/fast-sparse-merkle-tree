use fast_sparse_merkle_tree::{
    CompiledMerkleProof, H256, Hash, SparseMerkleTree, default_store::DefaultStore, traits::Hasher, turboshake_hasher::TurboShake128Hasher,
};

const MESSAGE: &str = "Alice sends Bob verifiable proofs using Sparse Merkle Tree";
const SMT_KEY_BYTE_LENGH: usize = 32;

type Smt = SparseMerkleTree<TurboShake128Hasher, Hash, H256, DefaultStore<Hash, H256, SMT_KEY_BYTE_LENGH>, SMT_KEY_BYTE_LENGH>;

fn main() {
    let kv_pairs: Vec<(Hash, H256)> = MESSAGE
        .split_whitespace()
        .enumerate()
        .map(|(idx, word)| {
            let key: Hash = {
                let mut hasher = TurboShake128Hasher::default();
                hasher.write_bytes(word.as_bytes());
                hasher.finish().into()
            };

            let value: H256 = {
                let mut hasher = TurboShake128Hasher::default();
                hasher.write_bytes(idx.to_le_bytes().as_ref());
                hasher.finish()
            };

            (key, value)
        })
        .collect();

    let mut smt = Smt::default();
    println!("Empty SMT root: {:?}", const_hex::encode(smt.root().as_slice()));

    for (key, value) in kv_pairs.iter() {
        smt.update(*key, *value).expect("Must be able to insert entry in SMT");
    }

    println!("Updated SMT root: {:?}", const_hex::encode(smt.root().as_slice()));

    for (key, expected_value) in kv_pairs.iter() {
        let lookup_value = smt.get(key).expect("Key must be present in SMT");
        assert_eq!(*expected_value, lookup_value);
    }

    let kv_pair = kv_pairs.first().expect("KV pair vector must be non-empty").to_owned();
    let alice_inclusion_proof = smt
        .merkle_proof(vec![kv_pair.0])
        .expect("Must be able to generate Merkle inclusion proof")
        .compile(vec![kv_pair])
        .expect("Must be able to serialize SMT inclusion proof")
        .0;
    println!("'Alice' word inclusion proof: {} bytes", alice_inclusion_proof.len());

    assert!(
        CompiledMerkleProof(alice_inclusion_proof)
            .verify::<TurboShake128Hasher, Hash, H256, SMT_KEY_BYTE_LENGH>(smt.root(), vec![kv_pair])
            .expect("Must be able to verify Merkle inclusion proof")
    );
    println!("'Alice' word inclusion proof is valid!");

    let kv_pair = {
        let word: &str = "Eve";
        let key: Hash = {
            let mut hasher = TurboShake128Hasher::default();
            hasher.write_bytes(word.as_bytes());
            hasher.finish().into()
        };
        let value: H256 = H256::zero();

        (key, value)
    };

    let eve_noninclusion_proof = smt
        .merkle_proof(vec![kv_pair.0])
        .expect("Must be able to generate Merkle non-inclusion proof")
        .compile(vec![kv_pair])
        .expect("Must be able to serialize SMT non-inclusion proof")
        .0;
    println!("'Eve' word non-inclusion proof: {} bytes", eve_noninclusion_proof.len());

    assert!(
        CompiledMerkleProof(eve_noninclusion_proof)
            .verify::<TurboShake128Hasher, Hash, H256, SMT_KEY_BYTE_LENGH>(smt.root(), vec![kv_pair])
            .expect("Must be able to verify Merkle non-inclusion proof")
    );
    println!("'Eve' word non-inclusion proof is valid!");
}
