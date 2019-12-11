use std::io::{self, Read};

fn main() -> io::Result<()> {
    let (w, h) = (25, 6);
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let layers = parse(buffer, w, h);
    let res1 = get_part1_solution(&layers);
    let image = combine_layers(&layers);
    let res2 = map_to_image(&image, w);
    println!("{:?}", res1);
    println!("{}", res2);
    Ok(())
}

fn map_to_image(image: &Vec<u32>, w: u32) -> String {
    image.iter()
         .map(|num| {
             match num {
                 0 => ".",
                 1 => "0",
                 _ => " ",
             }
         })
         .collect::<Vec<_>>()
         .chunks(w as usize)
         .map(|cnk| cnk.join(""))
         .collect::<Vec<_>>()
         .join("\n")
}

fn combine_layers(layers: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut res = layers[0].clone();
    layers[1..].iter()
               .for_each(|l| {
                   l.iter()
                    .enumerate()
                    .for_each(|(pos, &num)| {
                        if res[pos] == 2 {
                            res[pos] = num;
                        }
                    });
               });
    res
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
