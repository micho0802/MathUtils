use std::cmp::min;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}



pub fn factorial_of_number(number: i32) -> i32 {
    let mut identity: i32 = 1;
    for n in (1..=number) {
        identity *= n;
    }
    return identity
}

pub fn is_prime(number: i32) -> bool {
    if number < 1 {
        return false;
    }

    for n in (2..number) {
        if number % n == 0 {
            return false;
        }
    }
    true
}

// Greatest common divisor
pub fn gcd(number_1: i32, number_2: i32) -> i32 {
    let mut result: i32 = 1;
    let min = min(number_1, number_2);

    for n in (2..=min) {
        if number_1 % n == 0 && number_2 % n == 0 {result = n;}
    } return result
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_factorial() {
        let result: i32 = factorial_of_number(5);
        assert_eq!(result, 120);
    }

    #[test]
    fn test_is_prime() {
        let result: bool = is_prime(5);
        assert_eq!(result, true);
    }

    #[test]
    fn test_gcd() {
        let result: i32 = gcd(36, 54);
        assert_eq!(result, 18);
    }
}
