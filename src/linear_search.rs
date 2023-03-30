use std::cmp::Ordering;

pub fn linear_search(k: i32, items: &[i32]) -> Option<usize> {
    for (i, val) in items.iter().enumerate() {
        match val.cmp(&k) {
            Ordering::Equal => return Some(i),
            Ordering::Greater => continue,
            Ordering::Less => continue,
        }
    }
    None
}

#[test]
fn test_simple_search() {
    let items = vec![1, 2, 3, 4, 5];
    assert_eq!(Some(0), linear_search(1, &items));
    assert_eq!(Some(1), linear_search(2, &items));
    assert_eq!(Some(2), linear_search(3, &items));
    assert_eq!(Some(3), linear_search(4, &items));
    assert_eq!(Some(4), linear_search(5, &items));

    assert_eq!(None, linear_search(0, &items));
    assert_eq!(None, linear_search(90, &items));
    assert_eq!(None, linear_search(9000000, &items));
}
