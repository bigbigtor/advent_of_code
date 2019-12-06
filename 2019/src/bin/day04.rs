use std::io;

fn main() -> io::Result<()> {
    let (min, max) = read_input();
    let mut res1 = 0;
    let mut res2 = 0;
    (min..max).for_each(|n| {
        let digi_vec = digit_vec_from(n);
        let only_gte = only_gte(&digi_vec);
        let cont_adj = contains_same_adjacent(&digi_vec);
        let two_adj = only_two_adjacent(&digi_vec);
        if only_gte && cont_adj { res1 += 1; }
        if only_gte && two_adj { res2 += 1; }
    });
    println!("{}\n{}", res1, res2);
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

fn only_two_adjacent(digits: &Vec<u32>) -> bool {
    let mut counts = vec![0; 10];
    for d in digits {
        counts[*d as usize] += 1;
    }
    for c in counts {
        if c == 2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid1() {
        let num = 111111;
        let digi_vec = digit_vec_from(num);
        let only_gte = only_gte(&digi_vec);
        let cont_adj = contains_same_adjacent(&digi_vec);
        assert!(only_gte && cont_adj);
    }

    #[test]
    fn test_valid2() {
        let num = 223450;
        let digi_vec = digit_vec_from(num);
        let only_gte = only_gte(&digi_vec);
        let cont_adj = contains_same_adjacent(&digi_vec);
        assert!(!(only_gte && cont_adj));
    }
    
    #[test]
    fn test_valid3() {
        let num = 123789;
        let digi_vec = digit_vec_from(num);
        let only_gte = only_gte(&digi_vec);
        let cont_adj = contains_same_adjacent(&digi_vec);
        assert!(!(only_gte && cont_adj));
    }

    #[test]
    fn test_valid4() {
        let num = 112233;
        let digi_vec = digit_vec_from(num);
        let only_gte = only_gte(&digi_vec);
        let two_adj = only_two_adjacent(&digi_vec);
        assert!(only_gte && two_adj);
    }

    #[test]
    fn test_valid5() {
        let num = 123444;
        let digi_vec = digit_vec_from(num);
        let only_gte = only_gte(&digi_vec);
        let two_adj = only_two_adjacent(&digi_vec);
        assert!(!(only_gte && two_adj));
    }

    #[test]
    fn test_valid6() {
        let num = 111122;
        let digi_vec = digit_vec_from(num);
        let only_gte = only_gte(&digi_vec);
        let two_adj = only_two_adjacent(&digi_vec);
        assert!(only_gte && two_adj);
    }

}
