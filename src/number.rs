// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message

fn main() {
  for i in 1..=5 {
    for j in 1..i{
         print!("{}",j);
    }
    println!("{}",i);
 }
}


//

fn main() {
  for i in 1..=5 {
    for j in 1..i{
         print!("{}",i);
    }
    println!("{}",i);
 }
}

fn main() {
  let mut total = 5;

  for _ in 1..=5 {
    for j in 1..=total{
         print!("{}",j);
    }
     total-=1;
    println!("");
 }
}