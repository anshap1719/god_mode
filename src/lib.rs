#[macro_export]
macro_rules! god_mode {
    ($e:expr) => {{
        unsafe {
            $e()
        }
    }};
}
