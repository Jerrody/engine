#[macro_export]
macro_rules! log_msg {
    ($level:ident, $app_layer:expr, $app_name:expr, $env_layer:expr, $env_name:expr, $s:expr) => {
        #[cfg(all(feature = $app_layer, feature = $env_layer))]
        tracing::$level!(
            "\n{TAB_IN_SPACES}[{}][{}]\n{TAB_IN_SPACES}{}",
            $app_name,
            $env_name,
            $s
        );
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! debug {
    ($s:expr) => {
        log_msg!(debug, "engine", "ENGINE", "dev", "DEV", $s);
        log_msg!(debug, "editor", "EDITOR", "dev", "DEV", $s);
        log_msg!(debug, "application", "APPLICATION", "dev", "DEV", $s);
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! info {
    ($s:expr) => {
        log_msg!(info, "engine", "ENGINE", "dev", "DEV", $s);
        log_msg!(info, "engine", "ENGINE", "stable", "STABLE", $s);
        log_msg!(info, "engine", "ENGINE", "shipping", "SHIPPING", $s);
        
        log_msg!(info, "editor", "EDITOR", "dev", "DEV", $s);
        log_msg!(info, "editor", "EDITOR", "stable", "STABLE", $s);
        log_msg!(info, "editor", "EDITOR", "shipping", "SHIPPING", $s);
        
        log_msg!(info, "application", "APPLICATION", "dev", "DEV", $s);
        log_msg!(info, "application", "APPLICATION", "stable", "STABLE", $s);
        log_msg!(info, "application", "APPLICATION", "shipping", "SHIPPING", $s);
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! warning {
    ($s:expr) => {
        log_msg!(warn, "engine", "ENGINE", "dev", "DEV", $s);
        log_msg!(warn, "engine", "ENGINE", "stable", "STABLE", $s);
        log_msg!(warn, "engine", "ENGINE", "shipping", "SHIPPING", $s);
        
        log_msg!(warn, "editor", "EDITOR", "dev", "DEV", $s);
        log_msg!(warn, "editor", "EDITOR", "stable", "STABLE", $s);
        log_msg!(warn, "editor", "EDITOR", "shipping", "SHIPPING", $s);
        
        log_msg!(warn, "application", "APPLICATION", "dev", "DEV", $s);
        log_msg!(warn, "application", "APPLICATION", "stable", "STABLE", $s);
        log_msg!(warn, "application", "APPLICATION", "shipping", "SHIPPING", $s);
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! error {
    ($s:expr) => {
        log_msg!(error, "engine", "ENGINE", "dev", "DEV", $s);
        log_msg!(error, "engine", "ENGINE", "stable", "STABLE", $s);
        log_msg!(error, "engine", "ENGINE", "shipping", "SHIPPING", $s);
        
        log_msg!(error, "editor", "EDITOR", "dev", "DEV", $s);
        log_msg!(error, "editor", "EDITOR", "stable", "STABLE", $s);
        log_msg!(error, "editor", "EDITOR", "shipping", "SHIPPING", $s);
        
        log_msg!(error, "application", "APPLICATION", "dev", "DEV", $s);
        log_msg!(error, "application", "APPLICATION", "stable", "STABLE", $s);
        log_msg!(error, "application", "APPLICATION", "shipping", "SHIPPING", $s);
    };
}
