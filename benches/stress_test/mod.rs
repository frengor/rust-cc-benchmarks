use criterion::Criterion;
use rand::rngs::StdRng;
use rand::SeedableRng;

mod rust_cc;
mod gc;
#[cfg(feature = "zb-safe-gc")]
mod zb_safe_gc;
#[cfg(feature = "shredder")]
mod shredder;
#[cfg(feature = "broom")]
mod broom;
#[cfg(feature = "mgc")]
mod mgc;
mod bacon_rajan_cc;

pub fn stress_test(c: &mut Criterion) {
    let group = &mut c.benchmark_group("stress test");
    rust_cc::benchmark_stress_test(group, &mut create_rng());
    gc::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "zb-safe-gc")]
    zb_safe_gc::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "shredder")]
    shredder::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "broom")]
    broom::benchmark_stress_test(group, &mut create_rng());
    #[cfg(feature = "mgc")]
    mgc::benchmark_stress_test(group, &mut create_rng());
    bacon_rajan_cc::benchmark_stress_test(group, &mut create_rng());
}

fn create_rng() -> StdRng {
    StdRng::seed_from_u64(0xCAFE)
}
