/// Malth Ultis
/// # Example of factorial of number 5
/// ```
/// use MathUtils::factorial::factorial_of_number;
/// let a: i32 = factorial_of_number(5); 
/// ```



pub fn factorial_of_number(number: i32) -> i32 {
    let mut identity: i32 = 1;
    for n in (1..=number) {
        identity *= n;
    }
    return identity
}

