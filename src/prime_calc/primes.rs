use std::thread;
pub struct PrimesCalcSettings{
    pub RANGE_LIMIT_BOTTOM: u32,
    pub RANGE_LIMIT_TOP: u32,
    pub THREAD_RECORDS_LIMIT: u32,
    num_of_threads: u32,
    threads_to_handle: Vec<thread::JoinHandle<()>>,
    pub results: PrimesResults
}

// Will use this later instead of tuple
pub struct PrimeResult{
    pub the_suspect: u32,
    pub dividers: Vec<u32>,
}

impl PrimeResult{
    pub fn is_prime(&self) -> bool {
        self.dividers.len() == 0
    }
    pub fn init( the_suspect: u32, dividers: Vec<u32>) -> Self {
        Self{
            the_suspect,
            dividers
        }
    }
}

pub struct PrimesResults{
    pub p_results: Vec<PrimeResult>,
    // Zmienne do statystyk
    pub prime_distances: Vec<Distance>,

}

impl PrimesResults{
    pub fn init() -> Self {
        Self {
            p_results: Vec::new(),
            prime_distances: Vec::new(),
        }
    }
    pub fn add( &mut self, single_result: PrimeResult ) {
        self.p_results.push(single_result);
    }
    pub fn add_all( &mut self, all_result:&mut Vec<PrimeResult> ) {
        self.p_results.append(all_result);
    }
    pub fn calculate_distances(&mut self) {
        let mut single_distance = None;
        let mut last_prime_index = None;

        for i in 0..self.p_results.len() {
            let suspect = &self.p_results[i];
            if suspect.is_prime() {
                if single_distance != None && last_prime_index!= None { self.prime_distances.push( Distance{ prime_index: last_prime_index.unwrap(), distance_to_last: single_distance.unwrap() } ); }
                single_distance = Some(0);
                last_prime_index = Some(i);
            }
            single_distance = Some(single_distance.unwrap()+1);
        }

        println!("Distances: {:?}", self.prime_distances);
    }
}

#[derive(Debug)]
pub struct Distance{
    pub prime_index: usize,
    pub distance_to_last: usize,
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
            results: PrimesResults::init(),
        }
    }

    pub fn start_calc(&mut self) {
        let mut current_thread = 0;
        {
            let vec_threads: &mut Vec<thread::JoinHandle<()>> = &mut self.threads_to_handle;
            let thread_records_limit = self.THREAD_RECORDS_LIMIT;

            while current_thread<self.num_of_threads {
                vec_threads.push( thread::spawn(move || { // Jak toteraz wyciągnąć z tąd xD
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
    fn check_primes_in_range( start : u32, end : u32, ) -> Vec<PrimeResult> {
        let mut results_vec = Vec::new();

        for x in (start+1)..=end {
            let prime_check_resoult = PrimesCalcSettings::check_if_prime(x);
            println!("Number {} is {} a prime!{}", x, if prime_check_resoult.is_prime() {""} else {"not"}, if prime_check_resoult.is_prime() {"".to_string()} else {format!("\n !and! it has {} dividers which are:{:?}", prime_check_resoult.dividers.len(), prime_check_resoult.dividers)});
            results_vec.push( prime_check_resoult );
        }

        results_vec
    }

    // Remeber to fix this
    fn check_if_prime( suspect: u32) -> PrimeResult { //(bool, usize, Vec<u32>) {
        let mut dividers : Vec<u32> = Vec::new();

        for n in 2..suspect {
            if (suspect%n == 0) {
                dividers.push(n);
            }
        }

        //(dividers.len() == 0, dividers.len(), dividers)
        PrimeResult::init( suspect, dividers )
    }

    
    pub fn build_line_for_writer( single_line: PrimeResult ) -> String {
        "text".to_string()
    }

}