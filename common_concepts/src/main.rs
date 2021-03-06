fn main() {
        
    //
    // ─── VARIABLES AND MUTABILITY ───────────────────────────────────────────────────
    //

    // Variables are immutable by default
    let immutable_variable = 5;

    println!("The value of immutable_variable is: {}", immutable_variable);
    // immutable_variable = 6; 
    // Since x is immutable, this would throw an error


    //This variable will be mutable
    let mut mutable_variable = 5;

    println!("The value of mutable_variable is: {}", mutable_variable);
    //This will mutate the variable
    mutable_variable = 6;

    println!("The value of mutable_variable is: {}", mutable_variable);

    //Constants can only be declared once in the scope.
    const MAX_POINTS: u32 = 100_000;

    println!("The value of constant MAX_POINTS is: {}", MAX_POINTS);

    //The shadowed variable will be overriden by the previous one.
    //Shadowing is recreating the variable, while a 'mut will '
    let shadowed_variable = 5;

    let shadowed_variable = shadowed_variable + 1;

    let shadowed_variable = shadowed_variable * 2;

    println!("The value of shadowed variable is {}", shadowed_variable);

    //Shadowing allows for type changes. The first spaces is 
    let spaces = "   ";
    
    let spaces = spaces.len();

    println!("The number of spaces in the spaces variable is: {}", spaces);

    //
    // ─── DATA TYPES ─────────────────────────────────────────────────────────────────
    //

    //We need to provide the type to parse to. Would error without u32 type
    let parsed_number_from_string: u32 = "42".parse().expect("Not a number!");

    println!("parsed_number_from_string is: {}", parsed_number_from_string);


    //Integer literals

    // let integer_literal_decimal = 922_222;
    // let integer_literal_hex = 0xff;
    // let integer_literal_octal = 0o77;
    // let integer_literal_binary = 0b1111_0000;

    //Numeric Operations

    let calc = 5.0 + 3.0 - 3.5 / 2.2 * 4.0 % 5.0;

    println!("numeric operations calc is: {}", calc);

    //Boolean type

    let boolean_type: bool = true;

    println!("boolean type value {}", boolean_type );

    //Character type

    let heart_eyed_cat = '😻';
    
    println!("Character type: {}", heart_eyed_cat );

    //Compound types

    //Tuple type

    //let tuple: (i32, f64, u8) = (500, 6.4, 1);

    //Array type

    let array = [1, 2, 3, 4, 5];
    let index = 2;

    let element = array[index];

    println!("The value array element is: {}", element);

    //
    // ─── FUNCTIONS ──────────────────────────────────────────────────────────────────
    //

      println!("Return value of the function {}",  another_function(5));  

    //
    // ─── CONTROL FLOW ───────────────────────────────────────────────────────────────
    //

    let test_number = 3;

    //Basic if else
    if test_number != 0 {
        println!("number was something other than zero");
    } else if test_number % 3 == 0 {
        println!("number is divisible by 3");
    }

    //Conditional let statement
    let conditional_number = if test_number != 0 {
        5
    } else {
        0
    };

    println!("conditional_ number is: {}", conditional_number);

    //Loop statement

    let mut loop_variable = 0;

    loop {
        loop_variable = loop_variable + 1;
        println!("loop variable: {}", loop_variable);
        if loop_variable == 2 {
            break;
        }
    }

    //While statement

    let mut while_variable = 0;

    while while_variable < 5 {
        println!("while variable: {}", while_variable);
        while_variable = while_variable + 1;
    }

    let for_array = [10,20,30,40,50];

    for array_element in for_array.iter() {
        println!("array element: {}", array_element)
    }

    
}

fn another_function(x: i32) -> i32 {
    println!("The passed value is: {}", x);
    {
        let y = x + 2;
        y * 2
    }
    
}
