use digest::{dev::fixed_test, new_test};
use wasi_crypto_rust_crypto::hash::{Hash, Sha256, Sha512, Sha512_256};

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
