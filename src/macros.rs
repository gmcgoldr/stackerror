//! Provides a macro for formatting error messages with file and line information.

/// Formats a string using `format!`, and prefixes it with the file name and
/// line number.
#[macro_export]
macro_rules! fmt_loc {
    ($($arg:tt)*) => {{
        format!("{}:{} {}",
            file!(),
            line!(),
            format!($($arg)*)
        )
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fmt_lo() {
        let msg = fmt_loc!("Error {} occurred", 42);
        assert_eq!(msg, format!("src/macros.rs:20 Error 42 occurred"));
    }
}
