use test_macro::*;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn panic() {
    panic!();
}

test_assert!(test_assert, true);

test_assert_eq!(test_eq_test1, 1 + 2 => 3);
test_assert_eq!(test_eq_test2, add(1, 2) => 3);

test_assert_ne!(test_ne_test1, 1 + 2 => 0);
test_assert_ne!(test_ne_test2, add(1, 2) => 0);

test_should_panic!(test_should_panic1, panic!());
test_should_panic!(test_should_panic2, panic());
test_should_panic!(test_should_panic3, "FooBar", panic!("{}", String::from("FooBar")));