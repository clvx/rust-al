struct Cat(String);

// CatFeeder is a struct that holds a mutable reference to a Cat
// lifetime 'a is the lifetime of the reference to the Cat struct
struct CatFeeder<'a> {
    cat: &'a mut Cat
}

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

// CatFeeder has a method feed that calls the feed method on the Cat 
// that it holds a reference to
impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}


// The function longest takes two string slices and returns the longer one
// The lifetime 'a is the lifetime of the reference that is returned
// input references x and y must live at least as long as `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x // Output reference is tied to the longer input
    } else {
        y
    }
}

fn main() {
    // you cannot delete cats from the vector because it is borrowed mutably 
    // and used as a reference in the CatFeeder struct
    // To delete a cat, you need to remove the reference in the CatFeeder struct first.
    let mut cats = vec![
        Cat("Frodo".to_string()),
        Cat("Bilbo".to_string()),
        Cat("Pippin".to_string()),
    ];
    
    let mut feeders = Vec::new();
    // iter_mut() returns an iterator over mutable references
    // so we can't move the cats out of the vector
    // we need to borrow them instead
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder{ cat })
    }
    
    // we can't borrow the cats mutably here
    // because we are borrowing them mutably in the loop above
    // so we can't borrow them mutably again
    feeders.iter_mut().for_each(|f| f.feed());


    _ = longest("hello", "world");
}
