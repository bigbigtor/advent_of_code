use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let res1 : u32 = input.lines()
                          .filter_map(|s| s.parse::<u32>().ok())
                          .map(|mass| (mass/3) -2)
                          .sum();
    println!("{}", res1);
    Ok(())
}
