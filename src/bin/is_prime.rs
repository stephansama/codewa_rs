///
/// Define a function that takes an integer argument and returns a logical value true or false depending on if the integer is a prime.
///
/// Per Wikipedia, a prime number ( or a prime ) is a natural number greater than 1 that has no positive divisors other than 1 and itself.
///
/// Requirements
///
/// You can assume you will be given an integer input.
/// You can not assume that the integer will be only positive. You may be given negative numbers as well ( or 0 ).
/// NOTE on performance: There are no fancy optimizations required, but still the most trivial solutions might time out. Numbers go up to `2^31` ( or similar, depending on language ). Looping all the way up to n, or n/2, will be too slow.
/// ### Example
///
/// ```rust
/// is_prime(1) // false
/// is_prime(2) // true
/// is_prime(3) // false
/// ```
fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let answer = is_prime(12);
    println!("{}", answer);
    println!("Dawg")
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_negative_prime() {
        assert!(!is_prime(-5));
    }

    #[test]
    fn test_isnt_prime() {
        assert!(!is_prime(1));
        assert!(!is_prime(10));
        assert!(!is_prime(9));
        assert!(!is_prime(7912));
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2))
    }
}
