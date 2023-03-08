#[macro_export]
macro_rules! debug {
    ($s:expr) => {
        #[cfg(any(feature = "dev", feature = "editor", feature = "shipping"))]
        const DOUBLE_SPACE: &str = "  ";

        #[cfg(feature = "dev")]
        const DEV_NAME: &str = "DEV";

        #[cfg(feature = "dev")]
        tracing::debug!("\n{DOUBLE_SPACE}[{DEV_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
    };
}

#[macro_export]
macro_rules! info {
    ($s:expr) => {
        #[cfg(any(feature = "dev", feature = "editor", feature = "shipping"))]
        const DOUBLE_SPACE: &str = "  ";

        #[cfg(feature = "dev")]
        const DEV_NAME: &str = "DEV";
        #[cfg(feature = "editor")]
        const EDITOR_NAME: &str = "EDITOR";

        #[cfg(feature = "dev")]
        tracing::info!("\n{DOUBLE_SPACE}[{DEV_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
        #[cfg(feature = "editor")]
        tracing::info!("\n{DOUBLE_SPACE}[{EDITOR_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
    };
}

#[macro_export]
macro_rules! warning {
    ($s:expr) => {
        #[cfg(any(feature = "dev", feature = "editor", feature = "shipping"))]
        const DOUBLE_SPACE: &str = "  ";

        #[cfg(feature = "dev")]
        const DEV_NAME: &str = "DEV";
        #[cfg(feature = "editor")]
        const EDITOR_NAME: &str = "EDITOR";

        #[cfg(feature = "dev")]
        tracing::warn!("\n{DOUBLE_SPACE}[{DEV_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
        #[cfg(feature = "editor")]
        tracing::warn!("\n{DOUBLE_SPACE}[{EDITOR_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
    };
}

#[macro_export]
macro_rules! error {
    ($s:expr) => {
        #[cfg(any(feature = "dev", feature = "editor", feature = "shipping"))]
        const DOUBLE_SPACE: &str = "  ";

        #[cfg(feature = "dev")]
        const DEV_NAME: &str = "DEV";
        #[cfg(feature = "editor")]
        const EDITOR_NAME: &str = "EDITOR";
        #[cfg(feature = "shipping")]
        const SHIPPING_NAME: &str = "SHIPPING";

        #[cfg(feature = "dev")]
        tracing::error!("\n{DOUBLE_SPACE}[{DEV_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
        #[cfg(feature = "editor")]
        tracing::error!("\n{DOUBLE_SPACE}[{EDITOR_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
        #[cfg(feature = "shipping")]
        tracing::error!("\n{DOUBLE_SPACE}[{SHIPPING_NAME}]\n{DOUBLE_SPACE}{:?}", $s);
    };
}
