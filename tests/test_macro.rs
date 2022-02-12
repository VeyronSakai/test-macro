use test_macro::test_eq;
use test_macro::test_ne;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

test_eq!(test_eq_test1, 1 + 2 => 3);
test_eq!(test_eq_test2, add(1, 2) => 3);

test_ne!(test_ne_test1, 1 + 2 => 0);
test_ne!(test_ne_test2, add(1, 2) => 0);
