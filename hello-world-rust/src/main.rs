fn main() {
    println!("Hello, world!");

    // all variables in rust are immutable
   let x = "hello";
   println!("{}", x);

   // this would break
   // x = "world";

   let mut y = "hello";
   y = "world";
   println!("{}", y);

}
