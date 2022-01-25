fn main() {
    let mut num = 1;
    let mut ans = 0;
    let mut num1 = 1;
    let mut times = 1;

    println!("Made using Rust by Ajeeth");
    println!("What does it do? \nIts simple loop test multiplication.");
    while times < 11 {
    while num < 11 {
        ans = num1 * num;
        println!("{} * {} = {}", times,num,ans);
        num = num + 1;
       
    }
    print!("\n");
    num1 = num1 + 1;
    num = 1;
    times = times + 1;
    }

    }
    