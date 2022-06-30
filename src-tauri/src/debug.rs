#[macro_export]
macro_rules! println {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}
