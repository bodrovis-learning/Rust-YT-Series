use std::cmp::Ordering;

pub fn bsearch(arr: &[i32], desired_value: &i32) -> Option<(i32, usize)> {
    let mut low_bound: usize = 0;
    let mut up_bound: usize = arr.len() - 1;

    while low_bound <= up_bound {
        let mid = (up_bound + low_bound) / 2;

        let mid_value = arr[mid];

        match mid_value.cmp(desired_value) {
            Ordering::Equal => return Some((mid_value, mid)),
            Ordering::Greater => up_bound = mid.checked_sub(1)?,
            Ordering::Less => low_bound = mid + 1,
        }
    }

    None
}
