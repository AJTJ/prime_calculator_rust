fn main() {
    let mut stored_primes = vec![];
    fn check_if_prime(x: i32, primes: &mut Vec<i32>) -> bool {
        if x % 2 == 0 {
            return false;
        }

        for p in primes.iter() {
            if *p != 1 && x % p == 0 {
                return false;
            }
        }

        primes.push(x);
        true
    }

    let nums = 1..100;
    nums.for_each(|x| {
        println!(
            "is {} a prime number? {}",
            x,
            check_if_prime(x, &mut stored_primes)
        )
    });
    println!("final list of primes: {:?}", stored_primes);
}
