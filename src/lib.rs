
#[macro_export]
macro_rules! tu_simple_result {
    ($name:ident, $err:ty) => {
        pub type $name<T> = std::result::Result<T,$err>;
    }
}