//! This module provides a macro for formatting error messages with file and line information.

/// Formats a string using `format!`, and prefixes it with the file name and
/// line number.
#[macro_export]
macro_rules! stack_msg {
    ($($arg:tt)*) => {{
        format!("{}:{} {}",
            file!(),
            line!(),
            format!($($arg)*)
        )
    }}
}

/// Builds a closure that builds a new error from the given message,
/// prefixed with the file name and line number.
#[macro_export]
macro_rules! stack_else {
    ($type:ty, $($arg:tt)*) => {|| <$type>::new(stack_msg!($($arg)*))}
}

/// Builds a closure that maps an error and stacks the given message,
/// prefixed with the file name and line number.
#[macro_export]
macro_rules! stack_map {
    ($type:ty, $($arg:tt)*) => {|err| <$type>::new(err).stack_err(stack_msg!($($arg)*))}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stack_msg() {
        let msg = stack_msg!("Error {} occurred", 42);
        assert_eq!(msg, format!("src/macros.rs:34 Error 42 occurred"));
    }
}