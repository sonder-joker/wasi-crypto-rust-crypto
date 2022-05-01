use crate::raw::{
    OptOptions, OptOptionsUnion, OptSymmetricKey, OptSymmetricKeyUnion, Options,
    OPT_OPTIONS_U_NONE, OPT_OPTIONS_U_SOME, OPT_SYMMETRIC_KEY_U_NONE, OPT_SYMMETRIC_KEY_U_SOME, SymmetricKey,
};

pub fn to_c_opt_options(opt_options: Option<Options>) -> OptOptions {
    match opt_options {
        Some(options) => OptOptions {
            tag: OPT_OPTIONS_U_SOME.raw(),
            u: OptOptionsUnion { some: options },
        },
        None => OptOptions {
            tag: OPT_OPTIONS_U_NONE.raw(),
            u: OptOptionsUnion { none: () },
        },
    }
}

pub fn to_c_opt_opt_symmetric_key(opt_options: Option<SymmetricKey>) -> OptSymmetricKey {
    match opt_options {
        Some(options) => OptSymmetricKey {
            tag: OPT_SYMMETRIC_KEY_U_SOME.raw(),
            u: OptSymmetricKeyUnion { some: options },
        },
        None => OptSymmetricKey {
            tag: OPT_SYMMETRIC_KEY_U_NONE.raw(),
            u: OptSymmetricKeyUnion { none: () },
        },
    }
}
