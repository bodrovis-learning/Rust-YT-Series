#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
enum OrderStatus {
    Paid { amount: u32 },
    Sent,
    Delivered,
    Disputed(String),
}
impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Paid { amount } => println!("Paid, {amount}"),
            Self::Sent => println!("Sent"),
            Self::Delivered => println!("Delivered"),
            Self::Disputed(reason) => println!("Disputed with reason: {reason}"),
        };
    }
}

#[derive(Debug)]
struct Order {
    customer: String,
    status: OrderStatus,
}

fn main() {
    let status = OrderStatus::Sent;
    demo(&status);

    let order = Order {
        customer: String::from("John Doe"),
        status,
    };

    println!("{order:#?}");

    let status2 = OrderStatus::Disputed(String::from("broken"));
    println!("{status2:#?}");

    let paid = OrderStatus::Paid { amount: 100 };
    println!("{paid:#?}");
    paid.info();

    // Option
    //let value: Option<i8> = Some(32);
    let value: Option<i8> = None;
    println!("{value:#?}");

    // let result = value.unwrap_or(0) + 5;
    // println!("{result:#?}");

    // match value {
    //     Some(a) => {
    //         let result = a + 5;
    //         println!("{result}");
    //     },
    //     None => (),
    // }

    if let Some(a) = value {
        let result = a + 5;
        println!("{result}");
    } else {
        println!("5");
    }
}

fn demo(status: &OrderStatus) {
    println!("{status:#?}");
}
