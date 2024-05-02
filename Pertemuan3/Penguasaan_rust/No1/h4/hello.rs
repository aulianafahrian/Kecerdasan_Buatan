fn main(){
    // If Else Statement
    let num=20;
    if num==10{
        println!("num is equal to 10");
    }else{
        println!("num is not equal to 10");
    };

    // Let if statement
    let num = if true{
        100
    } else {
        200
    };
    println!("The value of num is {}", num);

    // Loop
    let mut num=5;
    loop{ //Loop statement
        println!("C# in {} Hours", num );
    if num == 8 {
        break; // Break statement
    }
        num=num+1;
    }

    // For Statement
    for num in 5..9{
        println!("Java in {} Hours", num);
    }

    // While Statement
    let mut num=0;
    while num<=8 {
        print!("{} ", num);
        num=num+1;
    }

    // Tuples
    let t = ("Python in",9,"Hours",true);
    println!("{} {} {} {}", t.0, t.1, t.2, t.3);

    // Match
    let num:i32 = 3; // given expression
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"), // match this
        4 => println!("four"),
        _ => println!("something else"),
    }
}
    