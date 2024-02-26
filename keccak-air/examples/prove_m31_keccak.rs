use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_circle::CirclePcs;
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_fri::FriConfig;
use p3_keccak::Keccak256Hash;
use p3_keccak_air::{generate_trace_rows, KeccakAir};
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_mersenne_31::Mersenne31;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher32};
use p3_uni_stark::{prove, verify, StarkConfig};
use rand::random;
use std::marker::PhantomData;
use tracing_forest::util::LevelFilter;
use tracing_forest::ForestLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

fn main() {
    let num_threads: usize = p3_maybe_rayon::prelude::current_num_threads();
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
        .with(ForestLayer::default())
        .init();

    type Val = Mersenne31;
    type Challenge = BinomialExtensionField<Val, 3>;

    type ByteHash = Keccak256Hash;
    type FieldHash = SerializingHasher32<ByteHash>;
    let byte_hash = ByteHash {};
    let field_hash = FieldHash::new(Keccak256Hash {});

    type MyCompress = CompressionFunctionFromHasher<u8, ByteHash, 2, 32>;
    let compress = MyCompress::new(byte_hash);

    type ValMmcs = FieldMerkleTreeMmcs<Val, u8, FieldHash, MyCompress, 32>;
    let val_mmcs = ValMmcs::new(field_hash, compress);

    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    type Challenger = SerializingChallenger32<Val, HashChallenger<u8, ByteHash, 32>>;

    type Pcs = CirclePcs<Val, ValMmcs, ChallengeMmcs>;

    type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;

    for n in (10..18).map(|k| 1 << k) {
        let num_perms = n / 24;

        let fri_config = FriConfig {
            log_blowup: 1,
            num_queries: 100,
            proof_of_work_bits: 16,
            mmcs: challenge_mmcs.clone(),
        };
        let pcs = Pcs {
            mmcs: val_mmcs.clone(),
            fri_config,
            _phantom: PhantomData,
        };
        let config = MyConfig::new(pcs);

        let mut challenger = Challenger::from_hasher(vec![], byte_hash);
        let inputs = (0..num_perms).map(|_| random()).collect::<Vec<_>>();

        let start = std::time::Instant::now();

        let trace = generate_trace_rows::<Val>(inputs);
        let proof = prove(&config, &KeccakAir {}, &mut challenger, trace, &vec![]);

        let t = start.elapsed();

        let mut challenger = Challenger::from_hasher(vec![], byte_hash);
        verify(&config, &KeccakAir {}, &mut challenger, &proof, &vec![]).unwrap();

        let tp = 1000f64 * num_perms as f64 / t.as_millis() as f64;
        println!("m+k+{num_threads}, perm: {num_perms}, time: {t:?}, throughtput: {tp:.02}");
    }
}
