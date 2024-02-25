#![warn(clippy::all, clippy::pedantic)]

use std::cell::RefCell;
use std::rc::Rc; // Interior mutability

#[derive(Clone)]
struct Cat {
    name: String,
    age: u8,
    parent: Option<Rc<RefCell<Cat>>>,
}

// reference counting -- Rc

fn main() {
    let cat_elder = Rc::new(RefCell::new(Cat {
        name: String::from("Elder Cat"),
        age: 16,
        parent: None,
    }));

    let cat_parent = Rc::new(RefCell::new(Cat {
        name: String::from("Parent Cat"),
        age: 6,
        parent: Some(Rc::clone(&cat_elder)),
    }));

    let cat = Rc::new(RefCell::new(Cat {
        name: String::from("Mr. Buttons"),
        age: 3,
        parent: Some(Rc::clone(&cat_parent)),
    }));

    parents_iterate(&cat);
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
