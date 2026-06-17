fn main() {
    let mut array = [10, 43, 12, 432, 54];
    let mut lowest = 0;

    println!("🚀 Start: {:?}\n", array);

    for outer in 0..array.len() {
        println!("🔍 Checking {}", array[outer]);

        for inner in 1..array.len() - 1 {
            print!(
                "   {} vs {} -> ",
                array[outer],
                array[inner]
            );

            if array[outer] > array[inner] {
                println!("skip");
                continue;
            }

            println!("update");

            if array[outer] > lowest {
                lowest = array[outer];
                println!("   ⭐ lowest = {}", lowest);
            }

            array[outer] = lowest;
        }

        println!("   📦 {:?}", array);
    }

    println!("\n🏁 Result: {:?}", array);
}