fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    const DATA1:&str = "987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn _1() {
        for i in DATA1.lines() {
            for j in i.chars() {
                println!("{}",j as i32);
            }
        }
    }
}
