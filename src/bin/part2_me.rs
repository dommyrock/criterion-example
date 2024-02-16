use std::collections::HashMap;

pub fn exec(lines:Vec<&str>) {
   let map = get_hmap();

   let mut sum = 0;
   for l in lines.iter() {
       let mut words_indices: Vec<(usize, u32)> = map
           .keys()
           .into_iter()
           .filter_map(|&x| l.find(x).and_then(|idx| Some((idx, *map.get(x).unwrap()))))
           .collect();

       let b = l.rmatch_indices(|c: char| c.is_numeric()).next();
       let f = l.match_indices(|c: char| c.is_numeric()).next();

       if let Some((idx, num)) = b {
           words_indices.push((idx, num.parse::<u32>().unwrap()));
       }
       if words_indices.len() == 1 || f != b {
           if let Some((idx, num)) = f {
               words_indices.push((idx, num.parse::<u32>().unwrap()));
           }
       }
       words_indices.sort_by(|a, b| a.0.cmp(&b.0));

       let r = format!(
           "{}{}",
           words_indices.first().unwrap().1,
           words_indices.last().unwrap().1
       );
       sum += r.parse::<u32>().unwrap();
   }
   // println!("Sum {sum}");
}
fn get_hmap() -> HashMap<&'static str, u32> {
   HashMap::from([
       ("one", 1),
       ("two", 2),
       ("three", 3),
       ("four", 4),
       ("five", 5),
       ("six", 6),
       ("seven", 7),
       ("eight", 8),
       ("nine", 9),
   ])
}

fn main() {
    let lines:Vec<_> =include_str!("../../input.txt").lines().collect();
    exec(lines);
}