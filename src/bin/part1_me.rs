pub fn exec(lines:Vec<&str>) {
   let mut sum = 0;
   for l in lines.iter() {
       let b = l
           .chars()
           .into_iter()
           .rev()
           .find_map(|c| c.is_numeric().then_some(c))
           .unwrap();
       let f = l
           .chars()
           .into_iter()
           .find_map(|c| c.is_numeric().then_some(c))
           .unwrap();
       let r = format!("{f}{b}").parse::<i32>().unwrap();
      //  println!("{r}");
       sum += r;
   }
   // println!("Sum {sum}");
}


fn main() {
    let lines:Vec<_> =include_str!("../../input.txt").lines().collect();
    exec(lines);
}