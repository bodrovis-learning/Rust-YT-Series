fn main() {
    let output_string = str_data();
    println!("{output_string}");
}

fn str_data() -> &'static str {
    "Hello, world!"
}

fn boom() {
    eprintln!("3... 2... 1...");
    panic!("boom!");
}

fn result(a: u8) -> Result<u8, String> {
    if a < 100 {
        Ok(a)
    } else {
        Err("invalid!".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_panic_when_a_is_too_large() -> Result<(), String> {
        let res = result(90);

        if res.is_err() {
            Err("Got an error!".into())
        } else {
            Ok(())
        }
    }

    #[test]
    #[should_panic(expected = "boom!")]
    fn it_always_panics() {
        boom();
    }

    #[test]
    #[ignore]
    fn it_returns_str() {
        assert_eq!(str_data(), "Hello", "Invalid string has been returned!")
    }

    #[test]
    fn it_returns_true() {
        // assert!(false);
        assert_eq!(true, true);
        assert_ne!(true, false);
    }
}
