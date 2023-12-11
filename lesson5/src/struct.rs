#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Car {
    brand: String,
    max_speed: u16,
    max_gas: f32,
    current_gas: f32,
    gas_consumption: f32,
}

impl Car {
    fn new(
        brand: &str,
        max_speed: u16,
        max_gas: f32,
        current_gas: f32,
        gas_consumption: f32,
    ) -> Self {
        Self {
            brand: String::from(brand),
            max_speed,
            max_gas,
            current_gas,
            gas_consumption,
        }
    }

    fn drive(&mut self, distance: f32) {
        let total_gas_consumed = distance * self.gas_per_km();

        if total_gas_consumed > self.current_gas {
            println!("Not enough gas!");
        } else {
            println!("Driving!");
            self.current_gas -= total_gas_consumed;
        }
    }

    fn gas_per_km(&self) -> f32 {
        self.gas_consumption / 100.0
    }

    fn faster(&self, other_car: &Car) -> bool {
        self.max_speed > other_car.max_speed
    }
}

// struct Color(i32, i32, i32);
// struct AlwaysEqual;

fn main() {
    let mut my_car = Car {
        brand: String::from("Ford"),
        max_speed: 180,
        max_gas: 55.0,
        current_gas: 55.0,
        gas_consumption: 23.0,
    };

    println!(
        "My car {} with max speed {} and max gas {}",
        my_car.brand, my_car.max_speed, my_car.max_gas
    );

    println!("{my_car:#?}");

    println!("Let's drive!");

    let distance = 40.0;

    // drive(&mut my_car, distance as f32);
    my_car.drive(distance);

    let car1 = Car::new("Volvo", 150, 55.0, 44.0, 23.0);
    println!("{car1:#?}");

    let car2 = Car {
        current_gas: 0.0,
        ..car1
    };

    println!("{car2:#?}");

    let is_faster = my_car.faster(&car2);
    println!("{is_faster}");
}

// fn drive(car: &mut Car, distance: f32) {
//     let gas_per_km = car.gas_consumption / 100.0;
//     let total_gas_consumed = distance * gas_per_km;

//     if total_gas_consumed > car.current_gas {
//         println!("Not enough gas!");
//     } else {
//         println!("Driving!");
//         car.current_gas -= total_gas_consumed;
//     }
// }