use std::io;

fn main() -> io::Result<()> {
    let (min, max) = read_input();
    let res: u32 = (min..=max).map(|n| if is_valid(n) {1} else {0})
                              .sum();
    println!("{}", res);
    Ok(())
}

fn read_input() -> (u32, u32) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let nums: Vec<u32> = buffer.trim()
                               .split("-")
                               .filter_map(|n| n.parse().ok())
                               .collect();
    (nums[0], nums[1])
}

fn digit_vec_from(num: u32) -> Vec<u32> {
    num.to_string().chars()
                   .map(|d| d.to_digit(10).unwrap())
                   .collect()
}

fn only_gte(digits: &Vec<u32>) -> bool {
    for i in 0..digits.len()-1 {
        if digits[i] > digits[i+1] {
            return false;
        }
    }
    true
}

fn contains_same_adjacent(digits: &Vec<u32>) -> bool {
    for i in 0..digits.len()-1 {
        if digits[i] == digits[i+1] {
            return true;
        }
    }
    false
}

fn is_valid(num: u32) -> bool {
    let digi_vec = digit_vec_from(num);
    contains_same_adjacent(&digi_vec) && 
        only_gte(&digi_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid1() {
        let num = 111111;
        assert!(is_valid(num));
    }

    #[test]
    fn test_valid2() {
        let num = 223450;
        assert!(!is_valid(num));
    }
    
    #[test]
    fn test_valid3() {
        let num = 123789;
        assert!(!is_valid(num));
    }
}
