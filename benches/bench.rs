#![feature(test)]

extern crate test;
extern crate wasm_wasda;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_wasda::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}