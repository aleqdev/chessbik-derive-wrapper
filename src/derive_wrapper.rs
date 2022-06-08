pub use derive_more;

#[macro_export]
macro_rules! derive_wrapper {
    ($i:item) => {
        #[derive(
            chessbik_derive_wrapper::derive_more::Deref,
            chessbik_derive_wrapper::derive_more::DerefMut,
            chessbik_derive_wrapper::derive_more::AsRef,
            chessbik_derive_wrapper::derive_more::From,
            chessbik_derive_wrapper::derive_more::Into,
        )]
        $i
    };
}
