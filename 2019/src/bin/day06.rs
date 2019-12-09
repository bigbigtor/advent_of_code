use std::io::{self, Read};
use std::collections::{HashSet, HashMap};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let orbits = parse(&buffer);
    let reverse_orbits = parse_reverse(&buffer);
    let res1 = get_num_orbits(&orbits, "COM", 0);
    let res2 = get_orbital_transfers(&reverse_orbits, "YOU", "SAN");
    println!("{}", res1);
    println!("{}", res2);
    Ok(())
}

fn get_orbital_transfers(
    reverse_orbits: &HashMap<String, String>,
    object1: &str,
    object2: &str
) -> u32 {
    let ob1_path = get_path(reverse_orbits, object1);
    let ob2_path = get_path(reverse_orbits, object2);
    ob1_path.symmetric_difference(&ob2_path).count() as u32
}

fn get_path(rev_orbits: &HashMap<String, String>, ob: &str) -> HashSet<String> {
    let mut path = HashSet::new();
    let mut current = ob;
    loop {
        if let Some(o) = rev_orbits.get(current) {
            path.insert(o.to_string());
            current = o;
        } else {
            break;
        }
    }
    path
}

fn get_num_orbits(
    orbits: &HashMap<String, Vec<String>>,
    object: &str,
    acc: u32
) -> u32 {
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

fn parse_reverse(input: &str) -> HashMap<String, String> {
    input.trim()
         .lines()
         .map(|s| {
             let split: Vec<&str> = s.split(")").collect();
             (split[1].to_string(), split[0].to_string())
         })
         .collect()
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

    #[test]
    fn test_orbit_transfers1() {
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
K)L
K)YOU
I)SAN";
        let rev_orbits = parse_reverse(&input);
        let res = get_orbital_transfers(&rev_orbits, "YOU", "SAN");
        let out = 4;
        assert_eq!(res, out);
    }

}
