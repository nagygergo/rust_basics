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

    
}
