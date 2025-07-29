mod prime_calc;

fn main() {

    let mut prime_calculation = prime_calc::primes::PrimesCalcSettings::init(2, 10000, 300);

    prime_calculation.start_calc();
    
}
