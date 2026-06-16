// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message

fn main() {
  let star = "*";
  for _ in 0..5 {
    for _ in 1..5{
         print!("{}",star);
    }
     println!("{}",star);
 }
}


// 

// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message

fn main() {
  let star = "*";
  for i in 0..5 {
    for j in 0..i{
         print!("{}",star);
    }
     println!("{}",star);
 }
}

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
