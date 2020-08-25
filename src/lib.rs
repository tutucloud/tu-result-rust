
#[macro_export]
macro_rules! simple_result {
    ($name:ident, $err:ty) => {
        pub type $name<T> = std::result::Result<T,$err>;
    }
}