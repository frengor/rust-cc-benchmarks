use criterion::Criterion;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[cfg(feature = "rust-cc")]
mod rust_cc;
#[cfg(feature = "gc")]
mod gc;
#[cfg(feature = "safe-gc")]
mod safe_gc;
#[cfg(feature = "zb-safe-gc")]
mod zb_safe_gc;
#[cfg(feature = "shredder")]
mod shredder;
#[cfg(feature = "broom")]
mod broom;
#[cfg(feature = "mgc")]
mod mgc;
#[cfg(feature = "bacon-rajan-cc")]
mod bacon_rajan_cc;

pub fn stress_test(c: &mut Criterion) {
    let group = &mut c.benchmark_group("stress test");
    #[cfg(feature = "rust-cc")]
    rust_cc::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "gc")]
    gc::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "bacon-rajan-cc")]
    bacon_rajan_cc::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "safe-gc")]
    safe_gc::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "zb-safe-gc")]
    zb_safe_gc::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "shredder")]
    shredder::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "broom")]
    broom::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "mgc")]
    mgc::benchmark_stress_test(group, &mut create_rng());
}

fn create_rng() -> StdRng {
    StdRng::seed_from_u64(0xCAFE)
}
