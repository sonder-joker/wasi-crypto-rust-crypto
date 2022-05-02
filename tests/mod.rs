use std::fmt::Debug;
use digest::{FixedOutput, new_test};
use wasi_crypto_rust_crypto::hash::{Hash, Sha256, Sha512, Sha512_256};

// Support `Copy` handle seem strange.
pub fn fixed_test<D>(input: &[u8], output: &[u8]) -> Option<&'static str>
    where
        D: FixedOutput + Default + Debug,
{
    let mut hasher = D::default();
    // Test that it works when accepting the message all at once
    hasher.update(input);
    if hasher.finalize_fixed()[..] != output[..] {
        return Some("whole message");
    }

    // Test that it works when accepting the message in chunks
    for n in 1..core::cmp::min(17, input.len()) {
        let mut hasher = D::default();
        for chunk in input.chunks(n) {
            hasher.update(chunk);
        }
        if hasher.finalize_fixed()[..] != output[..] {
            return Some("message in chunks");
        }
    }
    None
}

new_test!(sha256, "sha256", Hash::<Sha256>, fixed_test);
new_test!(sha512, "sha512", Hash::<Sha512>, fixed_test);
new_test!(sha512_256, "sha512_256", Hash::<Sha512_256>, fixed_test);

// TODO:current not work
// #[test]
// fn sha256_rand() {
//     let mut h = Hash::<Sha256>::new();
//     feed_rand_16mib(&mut h);
//     assert_eq!(
//         h.finalize()[..],
//         hex!("45f51fead87328fe837a86f4f1ac0eb15116ab1473adc0423ef86c62eb2320c7")[..]
//     );
// }
