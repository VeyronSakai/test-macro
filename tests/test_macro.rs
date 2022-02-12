use test_macro::test_eq;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

test_eq!(test_eq_test1, 1 + 2 => 3);
test_eq!(test_eq_test2, add(1, 2) => 3);
