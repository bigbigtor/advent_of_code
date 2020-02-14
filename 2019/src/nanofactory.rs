use std::collections::HashMap;

#[derive(Debug)]
struct Chemical {
    name: String,
    quantity: u64,
}

impl Chemical {
    fn new(name: &str, quantity: u64) -> Chemical {
        Chemical {
            name: String::from(name),
            quantity,
        }
    }
}

impl From<&str> for Chemical {
    fn from(input: &str) -> Chemical {
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let quantity = parts[0].parse().unwrap();
        let name = String::from(parts[1]);
        Chemical { name, quantity }
    }
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Chemical>,
    output: Chemical,
}

impl From<&str> for Reaction {
    fn from(line: &str) -> Reaction {
        let parts: Vec<&str> = line.split("=>").collect();
        let input = parts[0].split(",")
                            .map(Chemical::from)
                            .collect();
        let output = Chemical::from(parts[1]);
        Reaction { input, output }
    }
}

pub struct Nanofactory {
    reactions: HashMap<String, Reaction>,
}

impl Nanofactory {
    pub fn find_min_ore_qty(&self, quantity: u64) -> u64 {
        let mut ore = 0;
        let mut unprocessed = vec![Chemical::new("FUEL", quantity)];
        let mut storage: HashMap<String, u64> = HashMap::new();
        while let Some(Chemical {name, quantity}) = unprocessed.pop() {
            if name == "ORE" {
                ore += quantity;
            } else {
                match storage.get_mut(&name) {
                    Some(qty) if *qty >= quantity => {
                        *qty -= quantity;
                    },
                    _ => {
                        let reaction = &self.reactions[&name];
                        let stored = storage.entry(name).or_insert(0);
                        let min_produced = reaction.output.quantity;
                        let multiplier = ((quantity - *stored) as f64 / min_produced as f64).ceil() as u64;
                        *stored += min_produced * multiplier;
                        *stored -= quantity;
                        for ch in &reaction.input {
                            let needed = Chemical::new(&ch.name, ch.quantity * multiplier);
                            unprocessed.push(needed);
                        }
                    },
                }
            }
        }
        ore
    }
}

impl From<&str> for Nanofactory {
    fn from(input: &str) -> Nanofactory {
        let reactions = input.lines()
                             .map(|line| {
                                 let value = Reaction::from(line);
                                 let key = value.output.name.clone();
                                 (key, value)
                             })
                             .collect();
        Nanofactory { reactions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nanofactory_1() {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let output = 31;
        let factory = Nanofactory::from(input);
        assert_eq!(factory.find_min_ore_qty(1), output);
    }

    #[test]
    fn test_nanofactory_2() {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let output = 165;
        let factory = Nanofactory::from(input);
        assert_eq!(factory.find_min_ore_qty(1), output);
    }

    #[test]
    fn test_nanofactory_3() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let output = 13312;
        let factory = Nanofactory::from(input);
        assert_eq!(factory.find_min_ore_qty(1), output);
    }

    #[test]
    fn test_nanofactory_4() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let output = 180697;
        let factory = Nanofactory::from(input);
        assert_eq!(factory.find_min_ore_qty(1), output);
    }

    #[test]
    fn test_nanofactory_5() {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let output = 2210736;
        let factory = Nanofactory::from(input);
        assert_eq!(factory.find_min_ore_qty(1), output);
    }

}
