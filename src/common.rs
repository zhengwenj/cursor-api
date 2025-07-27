pub mod client;
// pub(crate) mod impls;
pub mod model;
pub mod time;
pub mod utils;

pub mod build {
    #[cfg(debug_assertions)]
    include!("../target/debug/build/build_info.rs");
    #[cfg(not(debug_assertions))]
    include!("../target/release/build/build_info.rs");
}
