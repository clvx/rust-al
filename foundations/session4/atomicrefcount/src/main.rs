use std::sync::Arc;

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

fn move_me(x: Arc<Droppable>) {
    //println!("Moved {}", x.0); // this is 1 for my_shared
    println!(" Atomic Reference count in move_me {}", Arc::strong_count(&x));
} // atomic reference count is decremented

fn main() {
    let my_shared = Arc::new(Droppable::new(1)); // Increment the reference
                                                                       // count to 1
    {
        let _x = my_shared.clone(); // Increment the reference count to 2
        let _y = my_shared.clone(); // Increment the reference count to 3
        let _z = my_shared.clone(); // Increment the reference count to 4
    } // _x, _y, _z go out of scope, reference count is decremented to 1
    move_me(my_shared.clone()); // Increment the reference count to 2
    println!(" Atomic Reference count after move_me {}", Arc::strong_count(&my_shared)); // 1

    let mut threads = Vec::new();
    for _ in 0.. 10 {
        let my_shared = my_shared.clone(); // Increment the reference count in i
    println!("Atomic Reference before spawning a new thread {}", Arc::strong_count(&my_shared));
        threads.push(std::thread::spawn(move || {
            move_me(my_shared)
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("Reference after threads terminate {}", Arc::strong_count(&my_shared));

    println!("{my_shared:?}");
    println!("Application exit");
}
