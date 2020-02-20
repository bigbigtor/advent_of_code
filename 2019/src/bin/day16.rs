use std::io::{self, Read};

fn main() -> io::Result<()> {
    let input = read_input()?;
    let res1 = apply_fft_algorithm(&input, 100);
    println!("{:?}", res1);
    let res2 = get_message(&input, 100);
    println!("{:?}", res2);
    Ok(())
}

fn get_message(signal: &Vec<i32>, num_phases: u8) -> u32 {
    let offset = signal.iter().take(7).fold(0, |acc, d| acc * 10 + d) as usize;
    let end = signal.len() * 10000;
    let mut message = (0..(end - offset)).map(|i| signal[(offset + i) % signal.len()])
                                         .collect::<Vec<i32>>();
    for _ in 0..num_phases {
        let mut sum: i32 = 0;
        for i in (0..(end-offset)).rev() {
            sum += message[i];
            message[i] = sum.abs() % 10;
        }
    }
    message.iter()
          .take(8)
          .map(|d| d.to_string())
          .collect::<Vec<_>>()
          .join("")
          .parse()
          .unwrap()
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

    #[test]
    fn get_message_1() {
        let output = 84462026;
        let signal = vec![0,3,0,3,6,7,3,2,5,7,7,2,1,2,9,4,4,0,6,3,4,9,1,5,6,5,4,7,4,6,6,4];
        assert_eq!(get_message(&signal, 100), output);
    }

    #[test]
    fn get_message_2() {
        let output = 78725270;
        let signal = vec![0,2,9,3,5,1,0,9,6,9,9,9,4,0,8,0,7,4,0,7,5,8,5,4,4,7,0,3,4,3,2,3];
        assert_eq!(get_message(&signal, 100), output);
    }

    #[test]
    fn get_message_3() {
        let output = 53553731;
        let signal = vec![0,3,0,8,1,7,7,0,8,8,4,9,2,1,9,5,9,7,3,1,1,6,5,4,4,6,8,5,0,5,1,7];
        assert_eq!(get_message(&signal, 100), output);
    }
}
