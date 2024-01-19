#![warn(clippy::all, clippy::pedantic)]

// fn sum<T: std::marker::Copy + std::ops::Add<Output = T>>(numbers: &[T]) -> T {
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }

fn sum<T>(numbers: &[T]) -> T
where
    T: std::marker::Copy + std::ops::Add<Output = T>,
{
    numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
}

// fn sum(numbers: &[i32]) -> i32 {
//     numbers.iter().sum()
// }

#[derive(Debug)]
struct Employee<T, U> {
    age: T,
    salary: U,
    tax: U,
}
impl<T, U: std::marker::Copy + std::ops::Sub<Output = U> + std::ops::Mul<Output = U>>
    Employee<T, U>
{
    fn salary_with_taxes(&self) -> U {
        self.salary - (self.salary * self.tax)
    }
}

trait Drive {
    fn can_drive(&self) -> bool;
}

struct Car {
    gas: u32,
}
impl Drive for Car {
    fn can_drive(&self) -> bool {
        self.gas > 0
    }
}

struct ElectroCar {
    battery_charge: u32,
}
impl Drive for ElectroCar {
    fn can_drive(&self) -> bool {
        self.battery_charge > 0
    }
}

fn car_info<T: Drive, U: Drive>(car: &T, other_car: &U) {
    println!("Can drive? {}", car.can_drive());
    println!("Can other car drive? {}", other_car.can_drive());
}

fn main() {
    let car = Car { gas: 10 };
    let electric_car = ElectroCar { battery_charge: 50 };
    car_info(&car, &electric_car);

    let employee = Employee {
        age: 21,
        salary: 5000.0,
        tax: 0.3,
    };

    println!("{employee:?}");

    let salary = employee.salary_with_taxes();
    println!("Salary with taxes {salary}");
    println!("Age {}", employee.age);

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let number_list2: Vec<i8> = vec![1, 2, 3, 4, 5];

    let result = sum(&number_list);
    println!("The sum is {result}");

    let result2 = sum(&number_list2);
    println!("The sum is {result2}");
}
