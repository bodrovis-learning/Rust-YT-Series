#![warn(clippy::all, clippy::pedantic)]

use std::rc::Rc;
// Interior mutability
use std::cell::RefCell;

#[derive(Clone)]
struct Cat {
    name: String,
    age: u8,
    parent: Option<Rc<RefCell<Cat>>>,
}

fn main() {
    let cat_elder = Rc::new(RefCell::new(Cat {
        name: "Matron".to_string(),
        age: 15,
        parent: None,
    }));

    println!(
        "count after creating elder = {}",
        Rc::strong_count(&cat_elder)
    );

    let mother = Rc::new(RefCell::new(Cat {
        name: "Mother".to_string(),
        age: 5,
        parent: Some(Rc::clone(&cat_elder)),
    }));

    println!(
        "count after creating elder = {}",
        Rc::strong_count(&cat_elder)
    );

    let buttons = Rc::new(RefCell::new(Cat {
        name: "Mr. Buttons".to_string(),
        age: 2,
        parent: Some(Rc::clone(&mother)),
    }));

    mother.borrow_mut().age = 6;
    buttons.borrow_mut().parent = Some(Rc::clone(&cat_elder));

    println!(
        "{} is {} years old",
        mother.borrow().name,
        mother.borrow().age
    );

    parents_iterate(&buttons);
}

fn parents_iterate(child: &Rc<RefCell<Cat>>) {
    let mut current = Some(Rc::clone(child));

    while let Some(current_cat) = current.take() {
        let current_cat_ref = current_cat.borrow();

        println!(
            "Getting cat named {}, age {}",
            current_cat_ref.name, current_cat_ref.age
        );

        if let Some(parent) = current_cat_ref.parent.as_ref() {
            let parent_ref = parent.borrow();

            println!(
                "{}'s parent is {} and its age is {}",
                current_cat_ref.name, parent_ref.name, parent_ref.age
            );

            current = Some(Rc::clone(parent));
        }

        println!("=====");
    }
}
