pub fn prime(number:i32) -> bool {
    let mut result = true;

    if number <= 3 {
        result = number > 1;
    } else if number % 2 == 0 || number % 3 == 0 {
        result = false;
    } else {
        let mut current = Some(5);

        while let Some(i) = current {
            if i * i > number {
                current = None;
            } else if number % i == 0 || number % (i + 2) == 0 {
                result = false;
                current = None;
            } else {
                current = Some(i + 6)
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_serivce_prime_true() {
        assert_eq!(super::prime(79), true);
    }
    
    #[test]
    fn test_serivce_prime_false() {
        assert_eq!(super::prime(4), false);
    }
}