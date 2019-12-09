use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let orbits = parse(&buffer);
    let res1 = get_num_orbits(&orbits, "COM", 0);
    println!("{}", res1);
    Ok(())
}

fn get_num_orbits(orbits: &HashMap<String, Vec<String>>, object: &str, acc: u32) -> u32 {
    match orbits.get(object) {
        Some(objects) => {
            objects.iter()
                   .map(|o| get_num_orbits(orbits, o, acc + 1))
                   .sum::<u32>() + acc
        },
        None => acc
    }
}

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input.trim()
         .lines()
         .map(|s| {
             let split: Vec<&str> = s.split(")").collect();
             (split[0], split[1])
         })
         .fold(HashMap::new(), |mut acc, (a,b)| {
             acc.entry(a.to_string())
                 .or_insert(Vec::new())
                 .push(b.to_string());
             acc
         })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbits1() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let orbits = parse(&input);
        let res = get_num_orbits(&orbits, "COM", 0);
        let out = 42;
        assert_eq!(res, out);
    }

}
