struct MyStruct {n: i32}

impl MyStruct {
    fn new( n: i32) -> Self {
        println!("Constructing {n}");
        Self { n }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {}", self.n);
    }
}

fn move_me(_x: MyStruct) {
    //Do nothing
}


// HasDroppable has transitive drop due to MyStruct
struct HasDroppables {
    _w: MyStruct,
}

fn main() {

    let x = MyStruct::new(1);
    let _z = MyStruct::new(3);
    {
        let _y = MyStruct::new(2);
    }

    let _has_drop = HasDroppables { _w: MyStruct::new(4) }; // drops first than _z
                                                                          // because it's first in
                                                                          // the stack
    move_me(x);
    println!("Back From the Function");
    println!("Ending the main function");
}
