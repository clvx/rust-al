use std::{fmt::Debug, rc::Rc};
use std::any::Any;

// Define a trait named Animal
trait Animal {
    fn speak(&self); // Define a method named speak that takes a reference to a struct that
                     // implements the Animal trait
}

struct Cat;

// Implement the Animal trait for the Cat struct
impl Animal for Cat {
    // Define the speak method for the Cat struct
    fn speak(&self) { 
        println!("Meow");
    }
}

// speak_twice takes a reference to a struct that implements the Animal trait
fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}

// get_animal returns a struct that implements the Animal trait
fn get_animal() -> impl Animal {
    Cat
}

trait DebuggableClonableAnimal: Animal+Debug+Clone {}

#[derive(Debug, Clone)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

// DowncastableAnimal is a trait that extends the Animal trait and the Any trait
// The Any trait is a trait that defines methods that allow dynamic casting of types
// The as_any method returns a reference to a dyn Any trait object
// The downcast_ref method returns an Option that contains a reference to the type that the dyn Any
// trait object was cast to 
// The downcast_mut method returns an Option that contains a mutable reference to the type that the
// dyn Any trait object was cast to 
// The downcast method returns a Result that contains the type that the dyn Any trait object was
// cast to 
// The DowncastableAnimal trait is implemented for the Tortoise struct 
// The as_any method returns a reference to the Tortoise struct as a dyn Any trait object 
trait DowncastableAnimal: Animal+Any {
    fn as_any(&self) -> &dyn Any;
}

struct Tortoise;

impl Animal for Tortoise {
    fn speak(&self) {
        println!("What noise does a tortoise make anyway?");
    }
}

// Implement the DowncastableAnimal trait for the Tortoise struct 
impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let cat = Cat; // Cat is a struct
    cat.speak(); // impl Animal for Cat

    speak_twice(&cat);

    let animal = get_animal(); // impl Animal for Cat

    // animals is a vector of Box<dyn Animal> (trait objects) that contains a Cat and a Dog 
    // (both structs that implement the Animal trait)
    // it needs to be a box because the size of the struct is not known at compile time
    // and the size of the struct is not known at compile time because the struct is a trait object
    let _animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];

    // downcasting is a way to cast a trait object to a concrete type
    // more_animals is a vector of Box<dyn DowncastableAnimal> (trait objects) that contains a
    // Tortoise (a struct that implements the DowncastableAnimal trait)
    let more_animals : Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
    for animal in more_animals.iter() {
        // if the animal is a Tortoise, print "We have access to the tortoise"
        // you have to know the type of the animal to downcast it
        if let Some(_t) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("We have access to the tortoise");
        }
        animal.speak();
    }
}
