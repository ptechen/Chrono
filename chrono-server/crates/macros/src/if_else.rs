#[macro_export]
macro_rules! if_else {
    ($condition:expr,$first:expr,$second:expr) => {{
        if $condition {
            $first
        } else {
            $second
        }
    }};
}
