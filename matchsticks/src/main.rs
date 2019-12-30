use std::io;
use std::mem;

fn main() {
    println!("*** Matchsticks Game Started ***");
    println!("Input the number of matchsticks.");
    let mut matchsticks = String::new();
    io::stdin().read_line(&mut matchsticks)
        .expect("Failed to read line");
    let mut matchsticks: u32 = matchsticks 
        .trim()
        .parse()
        .expect("Wanted a number");
    println!("Number of matchsticks inserted: {}", matchsticks);

    let mut done = false;
    let mut take = String::new();
    let mut player = 1;
    let mut opponent = 2;
    while !done {
        println!("Player {} turn.", player);
        io::stdin().read_line(&mut take)
            .expect("Failed to read line");

        let take_num: u32 = take 
            .trim()
            .parse()
            .expect("Wanted a number");

        assert!(take_num > 0);
        assert!(take_num <= 2);

        take.clear();
        matchsticks -= take_num;
        println!("Number of matchsticks taken: {}", take_num);
        println!("Number of matchsticks inserted: {}", matchsticks);
        if matchsticks == 1 {
          println!("Player {} won", player);
          done = true;
        }
        mem::swap(&mut player, &mut opponent);
    }
}
