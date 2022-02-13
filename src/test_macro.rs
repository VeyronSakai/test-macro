/// Generate a test code that internally uses assert_eq!.
///
/// The first argument should be the test case name,
/// and the second argument should be the arguments and the expected return value.
/// (Put the arguments to the left of => and the return value to the right)
///
/// # Example
///
/// ```
/// fn add(x: i32, y: i32) -> i32 {
///      x + y
/// }
/// ```
///
/// If you want to test this add function, you can write it as follows
///
/// ```
/// test_macro::test_eq!(test_case_name, add(1, 2) => 3);
/// ```
#[macro_export]
macro_rules! test_eq {
    ($func_name:ident, $arg:expr => $ans:expr) => {
        #[test]
        fn $func_name() {
            assert_eq!($arg, $ans);
        }
    };
}

#[macro_export]
macro_rules! test_ne {
    ($func_name:ident, $arg:expr => $ans:expr) => {
        #[test]
        fn $func_name() {
            assert_ne!($arg, $ans);
        }
    };
}
