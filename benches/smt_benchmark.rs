#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion, Throughput};
use nam_sparse_merkle_tree::{
    H256, Hash, default_store::DefaultStore, sha256_hasher::Sha256Hasher, tree::SparseMerkleTree,
    turboshake_hasher::TurboShake128Hasher,
};
use rand::Rng;

const TARGET_LEAVES_COUNT: usize = 20;

fn random_h256<R: Rng + ?Sized>(rng: &mut R) -> H256 {
    rng.random::<[u8; std::mem::size_of::<H256>()]>().into()
}

fn bench_sha256_smt(c: &mut Criterion) {
    type Sha256SMT = SparseMerkleTree<Sha256Hasher, Hash, H256, DefaultStore<Hash, H256, 32>, 32>;

    fn random_sha256_smt<R: Rng + ?Sized>(
        update_count: usize,
        rng: &mut R,
    ) -> (Sha256SMT, Vec<Hash>) {
        let mut smt = Sha256SMT::default();
        let mut keys = Vec::with_capacity(update_count);

        for _ in 0..update_count {
            let key = random_h256(rng);
            let value = random_h256(rng);

            smt.update(key.into(), value).unwrap();
            keys.push(key.into());
        }

        (smt, keys)
    }

    let mut group = c.benchmark_group("Sha256SMT update");
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut rng = rand::rng();
            b.iter(|| random_sha256_smt(size, &mut rng));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Sha256SMT get");
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _keys) = random_sha256_smt(size, &mut rng);

            b.iter(|| {
                let key = random_h256(&mut rng).into();
                smt.get(&key).unwrap();
            });
        });
    }
    group.finish();

    c.bench_function("Sha256SMT generate merkle proof", |b| {
        let mut rng = rand::rng();

        let (smt, mut keys) = random_sha256_smt(10_000, &mut rng);
        keys.dedup();

        let keys: Vec<_> = keys.into_iter().take(TARGET_LEAVES_COUNT).collect();
        b.iter(|| smt.merkle_proof(keys.clone()).unwrap());
    });

    c.bench_function("Sha256SMT verify merkle proof", |b| {
        let mut rng = rand::rng();
        let (smt, mut keys) = random_sha256_smt(10_000, &mut rng);
        keys.dedup();

        let leaves: Vec<_> = keys
            .iter()
            .take(TARGET_LEAVES_COUNT)
            .map(|k| (*k, smt.get(k).unwrap()))
            .collect();
        let proof = smt
            .merkle_proof(keys.into_iter().take(TARGET_LEAVES_COUNT).collect())
            .unwrap();

        let root = smt.root();
        b.iter(|| {
            assert!(
                proof
                    .clone()
                    .verify::<Sha256Hasher, Hash, H256, 32>(root, leaves.clone())
                    .expect("must pass verification")
            );
        });
    });

    let mut group = c.benchmark_group("Sha256SMT validate tree");
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _) = random_sha256_smt(size, &mut rng);

            b.iter(|| assert!(smt.validate()));
        });
    }
    group.finish();
}

fn bench_turboshake128_smt(c: &mut Criterion) {
    type TurboShake128SMT =
        SparseMerkleTree<TurboShake128Hasher, Hash, H256, DefaultStore<Hash, H256, 32>, 32>;

    fn random_turboshake128_smt<R: Rng + ?Sized>(
        update_count: usize,
        rng: &mut R,
    ) -> (TurboShake128SMT, Vec<Hash>) {
        let mut smt = TurboShake128SMT::default();
        let mut keys = Vec::with_capacity(update_count);

        for _ in 0..update_count {
            let key = random_h256(rng);
            let value = random_h256(rng);

            smt.update(key.into(), value).unwrap();
            keys.push(key.into());
        }

        (smt, keys)
    }

    let mut group = c.benchmark_group("TurboShake128SMT update");
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut rng = rand::rng();
            b.iter(|| random_turboshake128_smt(size, &mut rng));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("TurboShake128SMT get");
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _keys) = random_turboshake128_smt(size, &mut rng);

            b.iter(|| {
                let key = random_h256(&mut rng).into();
                smt.get(&key).unwrap();
            });
        });
    }
    group.finish();

    c.bench_function("TurboShake128SMT generate merkle proof", |b| {
        let mut rng = rand::rng();

        let (smt, mut keys) = random_turboshake128_smt(10_000, &mut rng);
        keys.dedup();

        let keys: Vec<_> = keys.into_iter().take(TARGET_LEAVES_COUNT).collect();
        b.iter(|| smt.merkle_proof(keys.clone()).unwrap());
    });

    c.bench_function("TurboShake128SMT verify merkle proof", |b| {
        let mut rng = rand::rng();
        let (smt, mut keys) = random_turboshake128_smt(10_000, &mut rng);
        keys.dedup();

        let leaves: Vec<_> = keys
            .iter()
            .take(TARGET_LEAVES_COUNT)
            .map(|k| (*k, smt.get(k).unwrap()))
            .collect();
        let proof = smt
            .merkle_proof(keys.into_iter().take(TARGET_LEAVES_COUNT).collect())
            .unwrap();

        let root = smt.root();
        b.iter(|| {
            assert!(
                proof
                    .clone()
                    .verify::<Sha256Hasher, Hash, H256, 32>(root, leaves.clone())
                    .expect("must pass verification")
            );
        });
    });

    let mut group = c.benchmark_group("TurboShake128SMT validate tree");
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _) = random_turboshake128_smt(size, &mut rng);

            b.iter(|| assert!(smt.validate()));
        });
    }
    group.finish();
}

criterion_group!(
    name = smt;
    config = Criterion::default().sample_size(100);
    targets = bench_sha256_smt, bench_turboshake128_smt
);
criterion_main!(smt);
