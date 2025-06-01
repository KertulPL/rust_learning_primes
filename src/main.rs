use std::thread;

fn main() {

    //
    const RANGE_LIMIT: u32 = 50000;
    const THREAD_RECORDS: u32 = 5000;
    //

    let mut num_of_threads = RANGE_LIMIT/THREAD_RECORDS;
    let mut current_thread = 0;

    let mut threads_to_handle = Vec::new();

    while current_thread<num_of_threads {

        threads_to_handle.push( thread::spawn(move || {
            checkPrimesInRange(current_thread*THREAD_RECORDS, (current_thread+1)*THREAD_RECORDS);
        }));

        current_thread += 1;
    }

    threads_to_handle.into_iter().for_each(|single_thread| { single_thread.join().unwrap() } );
    
}

fn checkPrimesInRange( start : u32, end : u32 ) {
    let mut current_suspect = 1u32;

    for x in start..end {
        let prime_check_resoult = checkIfPrime(x);
        println!("Number {} is {} a prime!{}", x, if prime_check_resoult.0 {""} else {"not"}, if prime_check_resoult.0 {"".to_string()} else {format!("\n !and! it has {} dividers which are:{:?}", prime_check_resoult.1, prime_check_resoult.2)});
    }
}

fn checkIfPrime( suspect: u32) -> (bool, usize, Vec<u32>) {
    let mut divider : Vec<u32> = Vec::new();

    for n in 2..suspect {
        if (suspect%n == 0) {
            divider.push(n);
        }
    }

    (divider.len() == 0, divider.len(), divider)
}
