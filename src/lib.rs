#[macro_export]
macro_rules! trust_me_its_safe {
    ($e:expr) => {{
        unsafe {
            $e()
        }
    }};
}
