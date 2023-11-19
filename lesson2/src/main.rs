#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

fn main() {
    let arr = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];

    let result = bin_search(&arr, -100);

    match result {
        Some((found_value, found_index)) => println!("Found value {found_value} at {found_index}"),
        None => println!("Not found!")
    }
}

fn bin_search(arr: &[i32], desired_value: i32) -> Option<(i32, usize)> {
    let mut low_bound: usize = 0;
    let mut up_bound: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low_bound <= up_bound {
        i += 1;

        let mid = (up_bound + low_bound) / 2;

        let mid_value = arr[mid];
        println!("{mid_value}");
        println!("{mid}");
        match mid_value.cmp(&desired_value) {
            Ordering::Equal => return Some((mid_value, mid)),
            Ordering::Greater => {
                up_bound = match mid.checked_sub(1) {
                    Some(result) => result,
                    None => return None // fallback
                };
            },

            // Lines 30-35 can be replaced with (thanks, @AndrewKraevskii):
            // Ordering::Greater => up_bound = mid.checked_sub(1)?,

            Ordering::Less => low_bound = mid + 1,
        }

        println!("Step {i}");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARR: [i32; 10] = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];

    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }

    #[test]
    fn element_not_found() {
        let result = bin_search(&ARR, 1234);

        assert!(result.is_none());
    }

    #[test]
    fn smallest_element_not_found() {
        let result = bin_search(&ARR, -100);

        assert!(result.is_none());
    }
}