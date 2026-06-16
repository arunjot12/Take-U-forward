// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message

fn main() {
  let star = "*";
  let mut total = 5;

  for i in 1..=5 {
    for j in 1..total{
         print!("{}",star);
    }
     total-=1;
    println!("{}",star);
 }

}
