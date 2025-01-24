use std::alloc::{Layout, alloc, dealloc};

struct SmartPointer<T> {
    ptr: *mut u8, // raw pointer to the allocated memory
    data: *mut T, // raw pointer to the allocated memory casted to a raw 
                        // pointer to type T
    layout: Layout // layout of the allocated memory
}

impl <T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");

        unsafe {
            let layout = Layout::new::<T>(); // Holds the size and alignment 
                                                     // information for type T. This 
                                                     // is necessary for both allocating 
                                                     // (alloc) and deallocating (dealloc) 
                                                     // the memory correctly.
            let ptr = alloc(layout); //raw pointer (*mut u8) to a block of 
                                              //memory allocated by alloc of the 
                                              //specified layout.
            SmartPointer {
                ptr, // raw pointer to the allocated memory
                data: ptr as *mut T, // raw pointer to the allocated memory 
                                          // casted to a raw pointer to type T
                layout // layout of the allocated memory
            }
        }
    }
    // set the value of the allocated memory to val of type T 
    fn set(&mut self, val: T) { 
        unsafe {
            *self.data = val; // dereference the raw pointer to type T and 
                                  // assign the value to it
        }
    }
    // get returns a reference to the value of the allocated memory
    fn get(&self) -> &T { 
                                
        unsafe {
            self.data.as_ref().unwrap() // dereference the raw pointer to type T 
                                              // and return a rust reference to it
        }
    }
}

impl <T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Deallocating memory from SmartPointer");
        unsafe {
            dealloc(self.ptr, self.layout); // it doesn't deallocate self.data 
                                                 // because it's not a raw pointer 
                                                 // allocated by alloc but a raw
                                                 // pointer casted from self.ptr 
                                                 // which is deallocated here.
                                                 // self.layout is necessary for dealloc 
                                                 // to know the size and alignment of
                                                 // the memory block to deallocate.
        }
    }
}

fn main() {
    let mut my_num = SmartPointer::<i32>::new(); // allocate memory for i32
    my_num.set(12); // set the value of the allocated memory to 12
    println!("my_num = {}", my_num.get()); // get the value of the allocated memory
}
