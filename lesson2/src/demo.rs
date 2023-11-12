fn main() {
  println!("Hello, world!");

  let a = 32 + 3;

  let b = -5;
  let result = demo(a, b);

  let mut counter = 0;

  let result2 = loop {
      // ...
      if counter == 10 {
          break 42;
      }
      
      counter += 1;
  };

  while counter < 100 {
      counter += 1;
  }

  let a = [10, 20, 30, 40, 50];

  for el in a {
      println!("The value {el}");
  }
}

// statement - instructions perform actions, do not return value
// expression - evaluate and return value
fn demo(a: u8, b: i32) -> bool {
  println!("Hello, world {a}!");

  if a > 10 {
      // truthy path
      true
  } else if b > 100 {
      false
  } else {
      true
  }
}