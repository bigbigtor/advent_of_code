use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut program = parse(buffer);
    program[1] = 12;
    program[2] = 2;
    let program = run(&program);
    println!("{}", program[0]);
    Ok(())
}

fn parse(input: String) -> Vec<usize> {
    input.split(',')
         .filter_map(|s| s.parse().ok())
         .collect()
}

fn run(program: &Vec<usize>) -> Vec<usize> {
    let mut result = program.clone();
    program.chunks(4)
           .take_while(|s| s[0] != 99)
           .for_each(|c| {
               let (opcode, src1, src2, dst) = (c[0], c[1], c[2], c[3]);
               match opcode {
                   1 => result[dst] = result[src1] + result[src2],
                   2 => result[dst] = result[src1] * result[src2],
                   o => panic!("Wrong opcode{}", o),
               }
           });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let input = vec![1,0,0,0,99];
        let output = vec![2,0,0,0,99];
        assert_eq!(run(&input), output);
    }

    #[test]
    fn test_run_2() {
        let input = vec![2,3,0,3,99];
        let output = vec![2,3,0,6,99];
        assert_eq!(run(&input), output);
    }

    #[test]
    fn test_run_3() {
        let input = vec![2,4,4,5,99,0];
        let output = vec![2,4,4,5,99,9801];
        assert_eq!(run(&input), output);
    }

    #[test]
    fn test_run_4() {
        let input = vec![1,1,1,4,99,5,6,0,99];
        let output = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(run(&input), output);
    }
}
