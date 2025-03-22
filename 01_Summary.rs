//Reading: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
//https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

fn main() {
    // 01 TODO: Add the missing keyword.
    let mut x = 5;
    println!("x has the value {x}");
    x = 6;



    println!("x has the value {x}");

    //04
    let mut x = 3;
    println!("Number {x}");
    x = 5; // Don't change this line
    println!("Number {x}");

    //.05  shadowing!
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);


    //1.6 Constants are always immutable. They are declared with the keyword `const`
    // instead of `let`. The type of Constants must always be annotated.


    println!("01.1 recap: variables are immutable by default. Used mut to  make x changeable");
    println!("01.2: completed variable value");
    println!("1.3: learned new way to declare variables");
    println!("1.4: used mut again");
    println!("1.5: Redeclare variable when changing types");
    println!("1.6: const must be annotated such as const x: i8   for signed 8-bit integer");
}