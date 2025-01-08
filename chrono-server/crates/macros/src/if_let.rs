#[macro_export]
macro_rules! if_let_err {
    ($func:expr) => {{
        if let Err(e) = $func {
            tracing::error!("{:?}", e);
        }
    }};
    ($func:expr, $comment:expr) => {{
        if let Err(e) = $func {
            tracing::error!("{:?}: {e}", $comment);
        }
    }};
}

#[macro_export]
macro_rules! let_some {
    ($func:expr, $err:expr) => {{
        let Some(data) = $func else {
            return Err($err);
        };
        data
    }};
    ($func:expr, $err:expr, $comment:expr) => {{
        let Some(data) = $func else {
            tracing::error!("{}: {}", $comment, $err);
            return Err($err);
        };
        data
    }};

    ($func:expr, $default: expr, $err:expr, $comment:expr) => {{
        let Some(data) = $func else {
            let Some(data) = $default {} else {
                tracing::error!("{}: {}", $comment, $err);
                return Err($err);
            };
            data
        };
        data
    }};
}

#[macro_export]
macro_rules! let_some_default {
    ($func:expr, $default: expr, $err:expr) => {{
        let Some(data) = $func else {
            let Some(data) = $default {} else {
                tracing::error!("{}" $err);
                return Err($err);
            };
            return data
        };
        data
    }};
}

#[macro_export]
macro_rules! let_ok {
    ($func:expr, $err:expr) => {{
        let Ok(data) = $func else {
            return Err($err);
        };
        data
    }};
}

#[macro_export]
macro_rules! let_ok_some {
    ($func:expr, $err:expr) => {{
        let Ok(Some(data)) = $func else {
            return Err($err);
        };
        data
    }};
}
