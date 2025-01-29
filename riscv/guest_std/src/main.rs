fn main() {
    const LIMIT: usize = 10000;

    // Print all primes up to LIMIT, naive implementation O(n^2)
    let mut num_prime = 0;
    for i in 2..=LIMIT {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        num_prime += if is_prime { 1 } else { 0 };
    }

    println!("num_prime = {num_prime}");

    // Print all primes up to LIMIT, sieve method
    // let mut is_prime = vec![true; LIMIT + 1];
    // is_prime[0] = false;
    // is_prime[1] = false;
    // for i in 2..=LIMIT {
    //     if is_prime[i] {
    //         for j in (i * 2..=LIMIT).step_by(i) {
    //             is_prime[j] = false;
    //         }
    //         println!("{}", i);
    //     }
    // }
}
