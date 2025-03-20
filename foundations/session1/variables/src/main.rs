fn double (x: i32) -> i32 {
    // This is also valid
    //return x * 2;
    x * 2
}

fn double_or_nothing (x: i32) -> i32 {
    if x > 0 {
        return x * 2;
    }
    //returns 0 if x <= 0
    0
}

// greet takes ownership of the string but doesn't return it
fn greet (s: String) {
    println!("Hello, {s}");
}


// saludar returns ownership of the string
fn saludar (s: String) -> String {
    println!("Hola, {s}");
    s
}

// borrowing
fn greet_borrow (s: &String) {
    println!("Hello, {s}");
}

fn great_borrow_mut (s: &mut String) {
    *s = format!("Hello, {s}"); //format! is like println! but returns a string
}

// read_line reads a line from the standard input and returns an input
fn read_line () -> String {
    let mut input: String = String::new(); //input buffer
    std::io::stdin().read_line(&mut input).expect("Stdin not working"); //read line from stdin and
                                                                        //prints error message if
                                                                        //it fails
    input.trim().to_string()
}

fn main() {
    
    // Immutable
    let n: i32 = 10;
    // Mutable
    let mut x: i32 = 20;
    x += 1;

    // Making immutable variable Mutable
    let y: i32 = 30;
    let y = y + 1;

    println!("n: {n}, x: {x}, y: {y}");

    // scope
    {
        let n: i32 = 40;
        println!("inside scope: n: {n}\n");
    }

    let z: i32 = double(10);
    println!("{}", z);
    println!("double or nothing: {}", double_or_nothing(10));
    println!("double or nothing: {}", double_or_nothing(0));

    let i: i32 = 33;
    let m: i32 = if i >= 50 {
        100
    } else {
        25
    };
    println!("m: {}", m);

    // unit type
    let j: () = {
        let _x : i32 = 10;
    };
    println!("unit type: {:?}", j);

    ////////////////////////////////////////
    // Move by default, except when you copy
    let name: String = "Hello".to_string();
    greet(name.clone()); //this works as we deep copied the name variable. Clone is slow though.
    greet(name);
    //greet(name); //this doesn't compile as the name variable moved ownership to the greet
    //function

    let surname: String = "Hola".to_string();
    let surname: String = saludar(surname); //variable shadowing
    saludar(surname); 

    //borrowing
    let last_name: String = "Smith".to_string();
    greet_borrow(&last_name);
    greet_borrow(&last_name);

    //borrowing mutable
    let mut apellido: String = "Kartz".to_string();
    great_borrow_mut(&mut apellido);
    println!("apellido: {apellido}");

    //read_line from stdin
    let input: String = read_line();
    println!("typed: [{input}]");


}
