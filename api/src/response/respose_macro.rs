#[macro_export]
macro_rules! try_or {
    ($expr:expr, $on_err:expr) => {
        match $expr {
            Ok(v) => v,
            Err(_e) => {
                // Se quiser, logue: tracing::error!(error = ?_e, "handler error");
                return $on_err;
            }
        }
    };
}
