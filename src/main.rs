use std::fs::File;

fn main() {

    //let mut primes_base = File::create("primes_container.csv");

    let mut current_suspect = 1u32;

    for x in 2..=500 {
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
