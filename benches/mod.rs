#![feature(test)]
extern crate test;

use digest::bench_update;
use wasi_crypto_rust_crypto::hash::{Hash, Sha256, Sha512, Sha512_256};
use test::Bencher;

bench_update!(
    Hash::<Sha256>::default();
    sha256_10 10;
    sha256_100 100;
    sha256_1000 1000;
    sha256_10000 10000;
);

bench_update!(
    Hash::<Sha512>::default();
    sha512_10 10;
    sha512_100 100;
    sha512_1000 1000;
    sha512_10000 10000;
);