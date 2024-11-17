//! This module provides a macro for formatting error messages with file and line information.

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_stack_msg() {
        let msg = stack_msg!("Error {} occurred", 42);
        assert_eq!(msg, format!("src/message.rs:18 Error 42 occurred"));
    }
}
