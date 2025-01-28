use std::collections::HashMap;

// Generic struct that holds a HashMap with a key of type K and a value of type Vec<V>
struct HashMapBucket<K,V>
{
    map: HashMap<K, Vec<V>>
}

// Implementation of the HashMapBucket struct
// K,V are generic types that implement the Eq and Hash traits
impl <K,V> HashMapBucket<K,V> 
where K: Eq + std::hash::Hash
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }

    // insert a key and value into the HashMap with the key of type K and the value of type Vec
    // if the key already exists in the HashMap, append the value to the existing Vec
    // otherwise, create a new Vec with the value and insert it into the HashMap
    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

fn main() {
    let mut my_buckets = HashMapBucket::new(); // create a new
                                                                         // HashMapBucket
    my_buckets.insert("hello", 1);
    my_buckets.insert("hello", 2);
    my_buckets.insert("goodbye", 3);
    println!("{:#?}", my_buckets.map); // print the HashMap
}
