fn factorial(num: u64) -> u64 {
    // Try not to use: - imperative style loops (for/while) - additional variables

    // Recursion
    // if num <= 1 { 1 } else { num * factorial(num - 1) }

    // I do like this one the most
    // (2..=num).fold(1, |acc, x| acc * x)

    (2..=num).product()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
