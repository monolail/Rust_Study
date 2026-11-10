
fn main() {
    println!("Hello, world!");

    println!("result = {}", get_sum(100));
}

fn get_sum(n: u32) -> u32{
    let mut sum:u32 = 0;

    for i in 1..=n{
        sum +=i;
    }

    return sum
}