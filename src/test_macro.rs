#[macro_export]
macro_rules! test_eq {
    ($func_name:ident, $arg:expr => $ans:expr) => {
        #[test]
        fn $func_name() {
            assert_eq!($arg, $ans);
        }
    }
}

#[macro_export]
macro_rules! test_ne {
    ($func_name:ident, $arg:expr => $ans:expr) => {
        #[test]
        fn $func_name() {
            assert_ne!($arg, $ans);
        }
    }
}
