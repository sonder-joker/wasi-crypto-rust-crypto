use crate::{
    adaptor::{to_c_opt_opt_symmetric_key, to_c_opt_options},
    raw,
};
use digest::{self, FixedOutput, HashMarker, OutputSizeUser, Update};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Sha256;
#[derive(Debug, Clone)]
pub struct Sha512;
#[derive(Debug, Clone)]
pub struct Sha512_256;

mod private {
    use digest::generic_array::{
        typenum::{U32, U64},
        ArrayLength,
    };

    pub trait Alg {
        type OutputSize: ArrayLength<u8> + 'static;
        fn name() -> &'static str;
    }

    impl Alg for super::Sha256 {
        type OutputSize = U32;

        fn name() -> &'static str {
            "SHA-256"
        }
    }

    impl Alg for super::Sha512 {
        type OutputSize = U64;

        fn name() -> &'static str {
            "SHA-512"
        }
    }

    impl Alg for super::Sha512_256 {
        type OutputSize = U32;

        fn name() -> &'static str {
            "SHA-512/256"
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hash<T: private::Alg> {
    handle: raw::SymmetricState,
    state: PhantomData<T>,
}

impl<T: private::Alg> HashMarker for Hash<T> {}

impl<T: private::Alg> Default for Hash<T> {
    fn default() -> Self {
        Self {
            handle: unsafe {
                raw::symmetric_state_open(
                    T::name(),
                    to_c_opt_opt_symmetric_key(None),
                    to_c_opt_options(None),
                )
                .unwrap()
            },
            state: PhantomData,
        }
    }
}

impl<T: private::Alg> Update for Hash<T> {
    fn update(&mut self, data: &[u8]) {
        unsafe {
            raw::symmetric_state_absorb(self.handle, data.as_ptr(), data.len()).unwrap();
        }
    }
}

impl<T: private::Alg> OutputSizeUser for Hash<T> {
    type OutputSize = T::OutputSize;
}

impl<T: private::Alg> FixedOutput for Hash<T> {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        let out = out.as_mut();
        unsafe {
            raw::symmetric_state_squeeze(self.handle, out.as_mut_ptr(), out.len()).unwrap();
        }
    }
}

impl<T: private::Alg> Drop for Hash<T> {
    fn drop(&mut self) {
        unsafe { raw::symmetric_state_close(self.handle).unwrap() }
    }
}
