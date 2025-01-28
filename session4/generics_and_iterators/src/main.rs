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

    // iter returns an iterator over the HashMap 
    // key_iter is an iterator over the keys of the HashMap 
    // current_map_entry is an Option that contains a reference to the current key and value in the
    // HashMap.
    fn iter(&self) -> HashMapBucketIter<K, V> {
        let mut key_iter = self.map.iter();
        let current_map_entry = key_iter.next();
        HashMapBucketIter {
            key_iter,
            current_map_entry,
            current_vec_index: 0,
        }
    }
}

struct HashMapBucketIter<'a, K, V> {
    key_iter: std::collections::hash_map::Iter<'a, K, Vec<V>>, // Iterator over the keys of the
                                                               // HashMap
    current_map_entry: Option<(&'a K, &'a Vec<V>)>, // Option that contains a reference to the
                                                     // current key and value in the HashMap
    current_vec_index: usize, // Index of the current value in the Vec
}

// Iterator trait implementation
// Specify 'a - the lifetime, and K,V on both sides.
// If you wanted to change how the iterator acts for a given type of key or
// value you could cange the left-hand side.
impl <'a, K, V> Iterator for HashMapBucketIter<'a, K, V> {
    // Define `Item` - a type used inside the trait - to be a reference to a key and value.
    // This specifies the type that the iterator will return.
    type Item = (&'a K, &'a V);

    // You use Item to specify the type returned by `Next`. It's always an option of the type.
    //
    // next returns an Option that contains a reference to the next key and value in the HashMap.
    // If there is a current map entry and a current vec index, and the index is less than the 
    // length of the vector, return the key and value at the current index. Otherwise, if the index 
    // is greater than or equal to the length of the vector, move to the next key in the HashMap. 
    // If there is a next key, return the key and value at the current index. Otherwise, return
    // None.
    fn next(&mut self) -> Option<Self::Item> {
        // If there is a current map entry, and a current vec index
        if let Some((key, values)) = self.current_map_entry {
            // If the index is less than the length of the vector
            if self.current_vec_index < values.len() {
                // Get the value at the current index
                let value = &values[self.current_vec_index];
                // Increment the index
                self.current_vec_index += 1;
                // Return the key and value
                return Some((key, value));
            } else {
                // We're past the end of the vector, next key
                self.current_map_entry = self.key_iter.next();
                self.current_vec_index = 0;

                if let Some((key, values)) = self.current_map_entry {
                    // If the index is less than the length of the vector
                    if self.current_vec_index < values.len() {
                        // Get the value at the current index
                        let value = &values[self.current_vec_index];
                        // Increment the index
                        self.current_vec_index += 1;
                        // Return the key and value
                        return Some((key, value));
                    }
                }
            }
        }

        None
    }
}


fn main() {
    let mut bucket = HashMapBucket::new(); // create a new
                                                                         // HashMapBucket
    bucket.insert("hello", 1);
    bucket.insert("hello", 2);
    bucket.insert("goodbye", 3);
    println!("{:#?}", bucket.map); // print the HashMap

    for (key, value) in bucket.iter() {
        println!("{}: {}", key, value);
    }
}
