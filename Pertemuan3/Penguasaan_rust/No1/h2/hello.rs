const NUM:i32 = 100;

// Function
fn funt(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

// Return Type
fn funt2(num:i32) -> i32 {
    num + 200
}

// Return Type Boolean
fn foo() -> bool {
    return true
}

fn main() {
    println!("Hello, world!");
    // Variable declaration
    let var = "OK";
    println!("The value of var is : {}", var);

    // Initialization with data type
    let nomor:i32 = 100;
    let str:String = "Good".to_string();
    println!("The value of nomor is : {}", nomor);
    println!("The value of str is : {}", str);

    // Output Format
    let x = 100;
    let y = 200;
    let z = 300;
    println!("{}",x);
    println!("{} {}",y,z);

    // Constant
    println!("The value of NUM is : {}", NUM);

    // Data Type Conversion
    let var1:f32 = 100.88;
    let var2:i32 = var1 as i32;
    println!("{}",var1);
    println!("{}",var2);

    // Function
    funt(100, 200);

    // Return Type
    let num = funt2(100);
    println!("The value of num is : {}", num);

    // Return Type Boolean
    let b = foo();
    println!("The result is: {}", b);
}
