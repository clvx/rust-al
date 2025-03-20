use std::{cell::RefCell, sync::Arc};

struct MyData {
    data: RefCell<String> // Wrap the data in a RefCell to ensure that only one thread can access the
                          // data at a time
}

impl MyData {
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".to_string()) // Initialize the data field with a RefCell
                                                    // containing the string shared between threads
                                                    // and wrapped in a RefCell
        }
    }
}

fn move_data(data: Arc<MyData>) {
    let mut data = data.data.borrow_mut(); // Borrow the data mutably to modify
                                                               // it in the current scope and lock
                                                               // the RefCell to access the shared_data
    data.push_str(" World"); // Append " World" to the shared data structure
}

fn main() {
    let shared_data = Arc::new(MyData::new()); // Increment the reference count
                                                                  // to 1
    move_data(shared_data.clone()); // Increment the reference count to 2
    let data = shared_data.data.borrow(); // Borrow the data immutably to access
                                                          // it in the current scope
    println!("{data}");
}
