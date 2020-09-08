fn main() {
    println!("Hello, world!");
    let x = 45; //dataType - i32 i.e. signed 32 bit integer
    let mut y = 50;
    let z: i64 = 1000; //dataType - i64 ie signed 64 bit
                       /*Some more data types
                           u64 - Unsigned 64 bit integer
                           f32 - floating type
                           bool - boolean
                       */
    println!("The value of x,y,z is {},{},{}", x, y, z);
    //x = 60 //Not possible as its immutable, by defualt in rust all are immutable
    y = 70; //This works as y has been given a mutable flag
    println!("New value of y is {}", y);
    //HOW TO USE IF STATEMENTS
    let mut n = 20;
    if n < 30 {
        println!("The number is less than 30");
    } else if n < 50 {
        println!("The number is less than 50");
    } else {
        println!("The number is greater than 45");
    }

    //loop

    loop {
        n += 1;
        if n > 100 {
            break;
        }
        println!("Inside loop ... {}", n);
    }
}

//This is an example single line comment

/*
    Hello from a multiline comment
*/
