use std::rc::Rc;

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn move_me(x: Rc<Droppable>) {
    println!("Moved {}", x.0);
    println!("Reference count in move_me {}", Rc::strong_count(&x)); // 2
}

fn main() {
    let my_shared = Rc::new(Droppable::new(1));
    {
        let _x = my_shared.clone(); // Increment the reference count
        let _y = my_shared.clone(); // Increment the reference count
        let _z = my_shared.clone(); // Increment the reference count
        println!("Reference count is {}", Rc::strong_count(&my_shared));
    } // _x, _y, _z go out of scope, reference count is decremented
    move_me(my_shared.clone()); // Increment the reference count
    println!("Reference count after move_me {}", Rc::strong_count(&my_shared)); // 1

    println!("{my_shared:?}");
    println!("Application exit");
}
