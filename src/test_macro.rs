/// Generate a test code that internally uses assert_eq!.
///
/// The first argument should be the test case name,
/// and the second argument should be the arguments and the expected return value.
///
/// Put the arguments to the left of "=>" and the return value to the right of "=>".
/// And panic if the left is not equal to the right.
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
/// test_macro::test_assert_eq!(test_case_name, add(1, 2) => 3);
/// ```
#[macro_export]
macro_rules! test_assert_eq {
    ($func_name:ident, $arg:expr => $ans:expr) => {
        #[test]
        fn $func_name() {
            assert_eq!($arg, $ans);
        }
    };
}

/// Generate a test code that internally uses assert_ne!.
///
/// The first argument should be the test case name,
/// and the second argument should be the arguments and the unexpected return value.
///
/// Put the arguments to the left of "=>" and the unexpected return value to the right of "=>".
/// And panic if the left is equal to the right.
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
/// test_macro::test_assert_ne!(test_case_name, add(1, 2) => 0);
/// ```
#[macro_export]
macro_rules! test_assert_ne {
    ($func_name:ident, $arg:expr => $ans:expr) => {
        #[test]
        fn $func_name() {
            assert_ne!($arg, $ans);
        }
    };
}

/// Test to see if it panics as expected.
///
/// The first argument should be the test case name,
/// and the second argument should be the function or macro which is expected to panic internally.
///
///
/// # Example
///
/// ```
/// test_macro::test_should_panic!(test_case_name, panic!());
/// ```
///
/// ```
/// fn panic() {
///     panic!();
/// }
///
/// test_macro::test_should_panic!(test_case_name, panic());
/// ```
#[macro_export]
macro_rules! test_should_panic {
    ($func_name:ident, $func:expr) => {
        #[test]
        #[should_panic]
        fn $func_name() {
            $func;
        }
    };
}