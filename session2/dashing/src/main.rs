use dashmap::DashMap;
use once_cell::sync::Lazy;

static SHARED: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

fn main() {
    for n in 0..100 {
        //100 threads are spawned. Each thread operates on its own n (from 0 to 99) 
        //due to the move keyword, which ensures the thread captures n by value.
        //Each thread runs in an infinite loop and attempts to:
        //- Increment the value for its unique key n if it exists.
        //- Insert a new entry (n, n) if it does not exist.
        std::thread::spawn(move || loop {
            if let Some(mut v)= SHARED.get_mut(&n){
                *v += 1;
            } else {
                SHARED.insert(n, n);
            }
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("{SHARED:#?}");
}
