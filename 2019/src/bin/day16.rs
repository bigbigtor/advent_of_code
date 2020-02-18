use std::io::{self, Read};

fn main() -> io::Result<()> {
    let input = read_input()?;
    let res1 = apply_fft_algorithm(&input, 100);
    println!("{:?}", res1);
    Ok(())
}

fn apply_fft_algorithm(signal: &Vec<i32>, num_phases: u8) -> u32 {
    let signal_len = signal.len();
    let pattern = get_pattern_matrix(signal_len);
    let mut signal = signal.clone();
    for _ in 0..num_phases {
        signal = apply_pattern(&signal, &pattern);
    }
    signal.into_iter()
          .take(8)
          .map(|d| d.to_string())
          .collect::<Vec<_>>()
          .join("")
          .parse()
          .unwrap()
}

fn apply_pattern(signal: &Vec<i32>, pattern: &Vec<i32>) -> Vec<i32> {
    signal.repeat(signal.len())
          .iter()
          .zip(pattern.iter())
          .map(|(&s, &p)| s * p)
          .collect::<Vec<_>>()
          .chunks(signal.len())
          .map(|c| c.iter().sum::<i32>().abs() % 10)
          .collect()
}

fn get_pattern_matrix(input_length: usize) -> Vec<i32> {
    (1..=input_length).into_iter()
                      .map(|n| get_nth_pattern(n, input_length))
                      .flatten()
                      .collect()
}

fn get_nth_pattern(n: usize, input_length: usize) -> Vec<i32> {
    [0, 1, 0, -1].iter()
                 .map(|i| [*i].repeat(n))
                 .flatten()
                 .cycle()
                 .skip(1)
                 .take(input_length)
                 .collect()
}

fn read_input() -> io::Result<Vec<i32>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.trim()
                      .chars()
                      .filter_map(|c| c.to_digit(10))
                      .map(|d| d as i32)
                      .collect();
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_pattern_1() {
        let output = [0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1];
        assert_eq!(get_nth_pattern(2, 15), output);
    }

    #[test]
    fn test_nth_pattern_2() {
        let output = [0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1];
        assert_eq!(get_nth_pattern(3, 11), output);
    }

    #[test]
    fn test_matrix_1() {
        let output = [1, 0, -1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1];
        assert_eq!(get_pattern_matrix(4), output);
    }

    #[test]
    fn test_apply_pattern_1() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let pattern = get_pattern_matrix(signal.len());
        let output = [4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(apply_pattern(&signal, &pattern), output);
    }

    #[test]
    fn apply_fft_1() {
        let output = 01029498;
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(apply_fft_algorithm(&signal, 4), output);
    }
}
