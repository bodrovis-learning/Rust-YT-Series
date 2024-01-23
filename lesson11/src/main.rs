#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Demo<'a> {
    field: &'a str,
}
impl<'a> Demo<'a> {
    fn call_me(&self, _s: &str) -> &str {
        self.field
    }
}

fn main() {
    // &'static
    // let a: &str = "long lifetime";

    let demo = Demo { field: "test" };
    println!("{demo:?}");
    let ret_val = demo.call_me("test");
    println!("{ret_val}");

    let v1 = vec![3, 4, 5];
    let v2 = vec![6, 7, 8];

    let greater_sum = compare_sums(&v1, &v2);

    for number in greater_sum {
        println!("{number}");
    }

    do_smth("demo");
}

// lifetime elision rules
fn do_smth(s: &str) -> &str {
    // &s[..]
    s
}

// fn sum<T>(numbers: &[T]) -> T
// where
//     T: std::marker::Copy + std::ops::Add<Output = T>,
// {
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }

fn sum<'a, T>(numbers: &'a [T]) -> T
where
    T: std::iter::Sum<&'a T>,
{
    numbers.iter().sum()
}

fn compare_sums<'a>(vector1: &'a Vec<i32>, vector2: &'a Vec<i32>) -> &'a Vec<i32> {
    if sum(vector1) >= sum(vector2) {
        vector1
    } else {
        vector2
    }
}
