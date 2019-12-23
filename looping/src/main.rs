fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value of a is {}", a[index]);
        index = index + 1;
    }

    // better soluzione the above can fail if index length is incorrect
    for element in a.iter() {
        println!("The value of a is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
    
    for number in 1..4 {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
