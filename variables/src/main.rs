fn main() {
    /* SCALARS
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is now: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
    */

    /* TUPLES
    let tup = ("Edan Barrios", 387);
    let checkings = tup.1;

    let error_codes = [202, 404, 500];
    let not_found = error_codes[1];
    */
    let sum= my_function(5, 10);
    println!("The sum of 5 and 10 is: {}", sum);

    //IF-ELSE STATEMENT
    if sum > 10 {
        println!("The sum is greater than 10");
    } else {
        println!("The sum is not greater than 10");
    }

    //BOOLEAN EXPRESSION
    let condition = true;
    let number = if condition {17} else {20};
    println!("The value of number is: {}", number);

    //FOR-LOOP
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value of element is: {}", element);
    }

    //RANGE
    for i in 1..=5 {
        println!("The value of i is: {}", i);
    }
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
