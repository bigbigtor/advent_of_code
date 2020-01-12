use std::collections::HashMap;

#[derive(Debug)]
struct Chemical {
    name: String,
    quantity: u64,
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
    pub fn find_min_ore_qty(&self) -> u64 {
        let mut result = 0;
        let mut demand: HashMap<String, u64> = HashMap::new();
        let mut cur_ch = "FUEL";
        for ch in &self.reactions[cur_ch].input {
            demand.entry(ch.name.to_string())
                  .and_modify(|e| *e += ch.quantity)
                  .or_insert(ch.quantity);
        }
        result
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
