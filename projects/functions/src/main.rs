fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    // Statement
    // let x = (let y = 6);

    // Expression. Aware of the semicolon at the end of expression.
    let y = {
        let x = 3;
        x + 1
    };

    let z = five();
    println!("The value of z is {z}");

    println!("The value of y is {y}");

    let w = plus_one(5);

    println!("The value of w is {w}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x+1; -> statement don't evaluate to a value. unit () returns.
}
