#[macro_export]
macro_rules! debug {
    ($s:expr) => {
        #[cfg(feature = "dev")]
        tracing::debug!("\n    [DEV]\n    {}", $s);
    };
}

#[macro_export]
macro_rules! info {
    ($s:expr) => {
        #[cfg(feature = "dev")]
        tracing::info!("\n    [DEV]\n    {}", $s);
        #[cfg(feature = "editor")]
        tracing::info!("\n    [EDITOR]\n    {}", $s);
    };
}

#[macro_export]
macro_rules! warning {
    ($s:expr) => {
        #[cfg(feature = "dev")]
        tracing::warn!("\n    [DEV]\n    {}", $s);
        #[cfg(feature = "editor")]
        tracing::warn!("\n    [EDITOR]\n    {}", $s);
    };
}

#[macro_export]
macro_rules! error {
    ($s:expr) => {
        #[cfg(feature = "dev")]
        tracing::error!("\n    [DEV]\n    {}", $s);
        #[cfg(feature = "editor")]
        tracing::error!("\n    [EDITOR]\n    {}", $s);
        #[cfg(feature = "shipping")]
        tracing::error!("\n    [SHIPPING]\n    {}", $s);
    };
}
