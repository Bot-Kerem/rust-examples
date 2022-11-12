#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

fn main()
{
    // mutability

    let immutable_var = 1;
    //immutable_var = 0; -> error
    // variables are immutable by default
    
    let mut mutable_var = 1;
    mutable_var = 0;
    println!("mutable var: {}", mutable_var);

    // scopes

    let long_lived_variable = 1;

    {   // inner block

        let short_lived_variable = 31;

        println!("short_lived_variable: {short_lived_variable}");
    }   // end of block

    // println!("short_lived_variable: {short_lived_variable}"); -> error
    // short_lived_variable doesn't exists any more short_lived_variable is deleted at end of block

    println!("long_lived_variable: {long_lived_variable}");

    // variable shadowing
    let mut shadowed_variable = 1;

    {   // inner block

        let mut shadowed_variable = 2;

        println!("inner shadowed_variable: {shadowed_variable}");

    }   // end of block

    println!("outer shadowed_variable: {shadowed_variable}");

    // declare first
    let a;

    {   // inner block

        a = 31; // effects outer scope

    }   // end of block

    println!("a: {a}");

    let uninitialized_variable: i32;

    // println!("uninitialized_variable: {uninitialized_variable}"); -> error
    // variable uninitialized

    // freezing

    let mut mutable_int = 31;

    {   // inner block

        let mutable_int = mutable_int; // becomes immutable in this scope. shadowing by immutable

        // mutable_int = 1; -> error
        // mutable_int is immutable

        println!("mutable_int: {mutable_int}");

    }   // end of block

    mutable_int = 0;
    
    println!("mutable_int: {mutable_int}");

    // const
    
    const const_variable: i32 = 5; // initialization in compile time
    let immutable_variable: i32 = 7; // initialization in run time
    // both of immutable

    // const_variable = 31; error -> variable is immutable!
    // immutable_variable = 31; error -> variable is immutable
    //
    println!("const_variable: {const_variable}");

    println!("immutable_variable: {immutable_variable}");
}
