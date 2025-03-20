fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids.into_iter().map(|core_id| {
        std::thread::spawn(move || {
            let success = core_affinity::set_for_current(core_id);
            if success { //this could fail as it's not guaranteed that the OS will allow you to set
                         //the affinity to the core you want
                println!("Thread running on core {core_id:?}");
            } else {
                println!("Failed to set core affinity for core {core_id:?}");
            }
        })
    }).collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}

