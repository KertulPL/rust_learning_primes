use std::thread;
pub struct PrimesCalcSettings{
    pub RANGE_LIMIT_BOTTOM: u32,
    pub RANGE_LIMIT_TOP: u32,
    pub THREAD_RECORDS_LIMIT: u32,
    num_of_threads: u32,
    threads_to_handle: Vec<thread::JoinHandle<()>>,
    results: Vec<(u32,Vec<u32>,bool)>
}

// Will use this later instead of tuple
pub struct PrimeResult{
    pub the_prime: u32,
    pub dividers: Vec<u32>,
}

impl PrimeResult{
    pub fn is_prime(&self) -> bool {
        self.dividers.len() == 1
    }
}

impl PrimesCalcSettings{
    pub fn init(RANGE_LIMIT_BOTTOM: u32, RANGE_LIMIT_TOP: u32, THREAD_RECORDS_LIMIT: u32) -> Self {

        let num_of_threads = (RANGE_LIMIT_TOP-RANGE_LIMIT_BOTTOM)/THREAD_RECORDS_LIMIT;

        Self{
            RANGE_LIMIT_BOTTOM,
            RANGE_LIMIT_TOP,
            THREAD_RECORDS_LIMIT,
            num_of_threads,
            threads_to_handle: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn start_calc(&mut self) {
        let mut current_thread = 0;
        {
            let vec_threads: &mut Vec<thread::JoinHandle<()>> = &mut self.threads_to_handle;
            let thread_records_limit = self.THREAD_RECORDS_LIMIT;

            while current_thread<self.num_of_threads {
                vec_threads.push( thread::spawn(move || {
                    PrimesCalcSettings::check_primes_in_range(current_thread*thread_records_limit, (current_thread+1)*thread_records_limit);
                }));

                current_thread += 1;
            }
        }

        for single_thread in &mut self.threads_to_handle.drain(..) {
            single_thread.join().unwrap();
        }
    }

    // Remeber to fix this
    fn check_primes_in_range( start : u32, end : u32, ) {
        let mut current_suspect = 1u32;

        for x in (start+1)..=end {
            let prime_check_resoult = PrimesCalcSettings::check_if_prime(x);
            println!("Number {} is {} a prime!{}", x, if prime_check_resoult.0 {""} else {"not"}, if prime_check_resoult.0 {"".to_string()} else {format!("\n !and! it has {} dividers which are:{:?}", prime_check_resoult.1, prime_check_resoult.2)});
        }
    }

    // Remeber to fix this
    fn check_if_prime( suspect: u32) -> (bool, usize, Vec<u32>) {
        let mut dividers : Vec<u32> = Vec::new();

        for n in 2..suspect {
            if (suspect%n == 0) {
                dividers.push(n);
            }
        }

        (dividers.len() == 0, dividers.len(), dividers)
    }
}