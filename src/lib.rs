pub mod search {
    use std::cmp::Ordering;

    pub fn linear(k: i32, items: &[i32]) -> Option<usize> {
        for (i, val) in items.iter().enumerate() {
            match val.cmp(&k) {
                Ordering::Equal => return Some(i),
                Ordering::Greater => continue,
                Ordering::Less => continue,
            }
        }
        None
    }

    pub fn binary(k: i32, items: &Vec<i32>) -> Option<usize> {
        let mut low: usize = 0;
        let mut high: usize = items.len();

        while low < high {
            let middle = (high + low) / 2;
            match items[middle].cmp(&k) {
                Ordering::Equal => return Some(middle),
                Ordering::Greater => high = middle,
                Ordering::Less => low = middle + 1,
            }
        }
        None
    }
}

pub mod sort {
    pub fn bubble<T: PartialOrd>(items: &mut [T]) {
        if items.len() < 2 {
            return;
        }

        for i in (0..items.len()).rev() {
            let mut has_swapped = false;
            for j in 0..i {
                if items[j] > items[j + 1] {
                    items.swap(j, j + 1);
                    has_swapped = true;
                }
            }

            if !has_swapped {
                break;
            }
        }
    }

    pub fn counting(arr: &mut [usize]) {
        let max_val = arr.iter().max().unwrap();
        let mut occurences = vec![0; max_val + 1];

        for &data in arr.iter() {
            occurences[data as usize] += 1;
        }

        let mut i = 0;
        for (data, &number) in occurences.iter().enumerate() {
            for _ in 0..number {
                arr[i] = data;
                i += 1;
            }
        }
    }

    pub fn selection<T: PartialOrd>(input: &mut [T]) {
        if input.len() < 2 {
            return;
        }

        for i in 0..input.len() {
            let swap_val = {
                let mut min = &input[i];
                let mut index_min = i;

                for j in i + 1..input.len() {
                    if input[j] < *min {
                        min = &input[j];
                        index_min = j;
                    }
                }
                index_min
            };

            if i != swap_val {
                input.swap(i, swap_val);
            }
        }
    }

    pub fn merge<T: PartialOrd + Copy>(input: &mut [T]) {
        if input.len() < 2 {
            return;
        }

        let len = input.len();
        let mid = len / 2;

        merge(&mut input[..mid]);
        merge(&mut input[mid..]);

        let mut tmp = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = mid;

        while i < mid && j < len {
            if input[i] < input[j] {
                tmp.push(input[i]);
                i += 1;
            } else {
                tmp.push(input[j]);
                j += 1;
            }
        }
        if i < mid {
            tmp.extend_from_slice(&input[i..mid]);
        } else if j < len {
            tmp.extend_from_slice(&input[j..len]);
        }

        input.copy_from_slice(&tmp[..]);
    }

    pub fn insertion<T: PartialOrd>(input: &mut [T]) {
        if input.len() < 2 {
            return;
        }

        for i in 1..input.len() {
            let mut j = i;
            while j > 0 && input[j - 1] > input[j] {
                input.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod search_tests {
    use super::search::*;

    #[test]
    fn test_binary_search() {
        let items: Vec<i32> = (0..=5).collect();

        assert_eq!(Some(0), binary(0, &items));
        assert_eq!(Some(1), binary(1, &items));
        assert_eq!(Some(2), binary(2, &items));
        assert_eq!(Some(3), binary(3, &items));
        assert_eq!(Some(4), binary(4, &items));

        assert_eq!(None, binary(90, &items));
        assert_eq!(None, binary(9000000, &items));
    }

    #[test]
    fn test_simple_search() {
        let items: Vec<i32> = (0..=5).collect();

        assert_eq!(Some(0), linear(0, &items));
        assert_eq!(Some(1), linear(1, &items));
        assert_eq!(Some(2), linear(2, &items));
        assert_eq!(Some(3), linear(3, &items));
        assert_eq!(Some(4), linear(4, &items));
        assert_eq!(Some(5), linear(5, &items));

        assert_eq!(None, linear(90, &items));
        assert_eq!(None, linear(9000000, &items));
    }
}

#[cfg(test)]
mod sort_tests {
    use super::sort::*;
    use rand::{seq::SliceRandom, thread_rng};

    #[test]
    fn test_bubble_sort() {
        let mut rng = thread_rng();
        let mut items: Vec<i32> = (1..=10000).collect();
        items.shuffle(&mut rng);

        bubble(&mut items);

        assert_eq!((1..=10000).collect::<Vec<i32>>(), items);
    }

    #[test]
    fn test_counting_sort() {
        let mut rng = thread_rng();
        let mut items: Vec<usize> = (0..=10000).collect();
        items.shuffle(&mut rng);

        counting(&mut items);

        assert_eq!((0..=10000).collect::<Vec<usize>>(), items);
    }

    #[test]
    fn merge_sort() {
        let mut rng = thread_rng();
        let mut items: Vec<i32> = (0..=10000).collect();
        items.shuffle(&mut rng);

        merge(&mut items);

        assert_eq!((0..=10000).collect::<Vec<i32>>(), items);
    }
}
