use std::thread;
#[derive(Clone)]
pub struct PrimesCalcSettings{
    pub RANGE_LIMIT_BOTTOM: u32,
    pub RANGE_LIMIT_TOP: u32,
    pub THREAD_RECORDS_LIMIT: u32,
    num_of_threads: u32,
    //current_thread: u32,
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
    pub fn init(&mut self, RANGE_LIMIT_BOTTOM: u32, RANGE_LIMIT_TOP: u32, THREAD_RECORDS_LIMIT: u32) {
        self.RANGE_LIMIT_BOTTOM = RANGE_LIMIT_BOTTOM;
        self.RANGE_LIMIT_TOP = RANGE_LIMIT_TOP;
        self.THREAD_RECORDS_LIMIT = THREAD_RECORDS_LIMIT;
        self.num_of_threads = (self.RANGE_LIMIT_TOP-self.RANGE_LIMIT_BOTTOM)/self.THREAD_RECORDS_LIMIT;
        //self.current_thread = 0;
        self.threads_to_handle = Vec::new();
        self.results = Vec::new();
    }

    pub fn do_calc(&mut self) {
        let mut current_thread = 0;
        let vec_threads: &mut Vec<thread::JoinHandle<()>> = &mut self.threads_to_handle;
        let thread_records_limit = self.THREAD_RECORDS_LIMIT;

        while current_thread<self.num_of_threads {
            vec_threads.push( thread::spawn(move || {
                PrimesCalcSettings::checkPrimesInRange(current_thread*thread_records_limit, (current_thread+1)*thread_records_limit);
            }));

            current_thread += 1;
        }
    

        self.threads_to_handle.into_iter().for_each(|single_thread| { single_thread.join().unwrap() } );
    }

    // Remeber to fix this
    fn checkPrimesInRange( start : u32, end : u32, ) {
        let mut current_suspect = 1u32;

        for x in (start+1)..=end {
            let prime_check_resoult = PrimesCalcSettings::checkIfPrime(x);
            println!("Number {} is {} a prime!{}", x, if prime_check_resoult.0 {""} else {"not"}, if prime_check_resoult.0 {"".to_string()} else {format!("\n !and! it has {} dividers which are:{:?}", prime_check_resoult.1, prime_check_resoult.2)});
        }
    }

    // Remeber to fix this
    fn checkIfPrime( suspect: u32) -> (bool, usize, Vec<u32>) {
        let mut dividers : Vec<u32> = Vec::new();

        for n in 2..suspect {
            if (suspect%n == 0) {
                dividers.push(n);
            }
        }

        (dividers.len() == 0, dividers.len(), dividers)
    }
}