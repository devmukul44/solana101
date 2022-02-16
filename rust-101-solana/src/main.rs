

fn main() {
    intro()
}

fn intro() {

    // 4 - Your First Rust Program
    // https://www.youtube.com/watch?v=c_cHO2qZmy4&list=PLeShFtA-ZIOVo7H59Gq-LA0Go1EiUs-vk&index=4
    println!("\n -- 4 - Your First Rust Program -- ");
    // macro
    println!("Hello, world!");

    // 5 - Introduction to Variables and Immutability
    // https://www.youtube.com/watch?v=c_cHO2qZmy4&list=PLeShFtA-ZIOVo7H59Gq-LA0Go1EiUs-vk&index=5
    println!("\n -- 5 - Introduction to Variables and Immutability -- ");

    // default - immutable variable
    let  x = 4;
    println!("{}", x);

    // mutable variable
    let mut y = 5;
    println!("{}", y);
    y = 6;
    println!("{}", y);


    // 6 - Types
    // https://www.youtube.com/watch?v=zqfxTXxtpec&list=PLeShFtA-ZIOVo7H59Gq-LA0Go1EiUs-vk&index=6
    println!("\n -- 6 - Types -- ");
    // variable holds some value of specific type!
    // Rust compiler supports type inference
    // Static Types - you cant change type of variable ones you have declared it! (even if variable is mutable)
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    // i8 -> signed integer of 8 bits, -128..=127
    // first bit is used for sign and last seven bits are used for data, 2^7 = 128
    let a: i8 = 127;
    println!("{}", a);

    print_type_of(&a);
    print_type_of(&x);

    let b = true;
    print_type_of(&b);

    let c = 22.0/7.0;
    print_type_of(&c);

    let s1 = "hello";
    print_type_of(&s1);

    let s2 = String::from("hello");
    print_type_of(&s2);


    // 7 - Composite Types
    // https://www.youtube.com/watch?v=WxYjmu4RoOE&list=PLeShFtA-ZIOVo7H59Gq-LA0Go1EiUs-vk&index=7
    println!("\n -- 7 - Composite Types -- ");

    //Tuple - set of values of different type
    let t1 = (1.0, "hello", true);
    println!("{}", t1.0);

    // assigining tuple values through destructuring
    let (_, t1_2, t1_3) = t1;
    println!("{} - {}" , t1_2, t1_3);

    // Arrays - set of values of same type
    let a1 = [1,2,3,4,5];
    println!("{}", a1[2]);

    let a2: [i32; 10] = [5; 10];
    println!("{:?}", a2);

    let mut a3: [i32; 10] = [5; 10];
    a3[1] = 0;
    println!("{:?}", a3);


    // 8 - Control Flow
    // https://www.youtube.com/watch?v=cO-JC2e9sXk&list=PLeShFtA-ZIOVo7H59Gq-LA0Go1EiUs-vk&index=8
    println!("\n -- 8 - Control Flow -- ");

    // if-else ladder
    let num = 1;

    if num == 0 {
        println!("you won");
    } else if num == 1 {
        println!("try again")
    } else {
        println!("you lost")
    }


    // 9 - Loops
    // https://www.youtube.com/watch?v=cO-JC2e9sXk&list=PLeShFtA-ZIOVo7H59Gq-LA0Go1EiUs-vk&index=9
    println!("\n -- 9 - Loops -- ");
    
    // loop
    let mut i = 1;
    loop {
        if i >= 5000 {
            break;
        }
        i = i * 2;
        print!("{},", i);
    }
    println!();
    println!("outside loop");

    // while loop
    i = 1;
    while i < 5000 {
        i = i * 2;
        print!("{},", i)
    }
    println!();
    println!("outside loop");
    
    // for loop
    for i in 0..10 {
        print!("{},", i)
    }
    println!();
    println!("outside loop");

    for i in 0..=10 {
        print!("{},", i)
    }
    println!();
    println!("outside loop");

    let iter1 = [0;10];
    for i in iter1 {
        print!("{},", i);
    }
    println!();
    println!("outside loop");


    // 10 - Match Statement
    // https://www.youtube.com/watch?v=cO-JC2e9sXk&list=PLeShFtA-ZIOVo7H59Gq-LA0Go1EiUs-vk&index=10
    println!("\n -- 10 - Match Statement -- ");

    let x1 = 23;
    match x1 {
        1 => println!("value is 1"),
        2 => println!("value is 2"),
        _ => println!("invalid value")
    }

}