mod prime_calc;
use std::thread;

fn main() {

    let mut prime_calculation = prime_calc::primes::PrimesCalcSettings::init(2, 10000, 300);

    prime_calculation.start_calc();
    
}


fn build_line_for_writer( single_line: (u32,bool, usize, Vec<u32>) ) -> String {
    "text".to_string()
}