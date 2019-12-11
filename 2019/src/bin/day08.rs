use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let layers = parse(buffer, 25, 6);
    let res1 = get_part1_solution(&layers);
    println!("{:?}", res1);
    Ok(())
}

fn get_part1_solution(layers: &Vec<Vec<u32>>) -> u32 {
    layers.iter()
          .map(|v| {
              let mut acc = [0; 3];
              for value in v {
                  acc[*value as usize] += 1;
              }
              (acc[0], acc[1] * acc[2])
          })
          .min_by_key(|(k, _)| *k)
          .unwrap()
          .1
}

fn parse(input: String, img_w: u32, img_h: u32) -> Vec<Vec<u32>> {
    let layer_len = img_w * img_h;
    input.trim()
         .chars()
         .map(|c| c.to_digit(10).unwrap())
         .collect::<Vec<_>>()
         .chunks(layer_len as usize)
         .map(|cnk| cnk.to_vec())
         .collect()
}
