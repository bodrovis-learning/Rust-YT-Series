#![warn(clippy::all, clippy::pedantic)]

fn main() {
    // iterators
    let v1: Vec<i32> = vec![1, 2, 3];

    // for i in v1.iter() {
    //     println!("{i}");
    // }

    let mut i = v1.iter();

    let v2: Vec<i32> = i.map(|x| x + 1).take(2).collect();

    for i in v2 {
        println!("{i}");
    }

    let v = vec!["zero", "one", "two", "three", "four", "five"];

    let result = v
        .iter()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 1)
        .map(|(i, _)| i * i)
        .take(3)
        .fold(0, |result, i| result + i);

    println!("{result}");
    // // closures
    // let param = 10;

    // let value: Option<u8> = None;

    // // let res: u8 = value.unwrap_or_else(|| -> u8 {
    // //     fallback(param)
    // // });

    // let closure = || -> u8 { fallback(param) };

    // let res: u8 = value.unwrap_or_else(closure);

    // closure();

    // let mut b: u8 = 0;

    // let mut demo = |a| {
    //     println!("{a}");
    //     b = b + a;
    // };

    // demo(3);

    // println!("{b}");

    // let mut v = vec![1,2];

    // let mut process = || v.push(3);

    // process();

    // v;
}

fn fallback(param: u8) -> u8 {
    42 + param
}
