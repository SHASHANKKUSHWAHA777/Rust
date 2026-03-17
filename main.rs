fn main() {
    println!("Hello, world!");
    let name = "john" ; 
    print!("name : {}\n",name);
    let age = 55;
    println!("{} is {} years old",name,age);

    /*let x=5;
    x=10;
    print!("{}",x); this particular code would give error as even if we used let to declare x , we cannot change the value of x later 
    to change the value of x later , we must use "mut" after let and before x , mut=mutable r changeable*/
    let mut x = 5;
    println!("before : {}",x);
    x=10;
    println!("After : {}",x);

    let myname = "shashank" ;
    println!("{}",myname);
    //if you want to specify explicitly the data type
    let myname2 :&str = "Shashank" ;
    println!("{}",myname2);
    let mynum : i32 = 45; //to specify integer we use i32 and for float we use f64
    println!("{}",mynum);

    //CONSTANTS - we have to specify its data type or it wont work
    /*const A = 4;
    println!("{}",A); this would have given an error as we are using const and data type is not defined */

    const A : i32 = 4;
    println!("{}",A);

    //rust operators
    let add = 5 + 3;
    let sub = 5-3;
    println!("Add : {}",add);
    println!("Sub : {}",sub);
    
    //assignment operators
    let mut x = 5;
    println!("Before : {}",x);
    x+=5;
    println!("After : {}",x);

    //comparison operator
    let a = 6;
    let b = 6;
    println!("6 == 6: {}", a==b);
    println!("6 < 6: {}", a<b);
    println!("6 != 6: {}", a!=b);

    //logical operators
    let isloggedin = true;
    let isadmin = false;
    println!("is regular user : {}",isloggedin && isadmin);
    println!("has any access : {}",isloggedin || isadmin);
    println!("not logged in : {}",!isloggedin);

     //rust booleans
    let isloggedin1 =  true;
    println!("is user loggein in? : {}",isloggedin1);

    let age = 20;
    let can_vote = age>= 18;
    println!("can vote? : {}",can_vote);

    //boolean if statements
    let userloggedin = true;
    if userloggedin {
        println!("Welcome");
    }
    else{
        println!("Please Login");
    }

    //rust if 
    if 7>5 {
        println!("7 greater than 5");
    }

    //rust if else
    let userloggedin2 = true;
    if userloggedin2 {
        println!("Welcome");
    }
    else{
        println!("Please Login");
    }

    //rust else if
    //You can check multiple conditions using else if:

    let score = 85;

    if score >= 90 {
    println!("Grade: A");
    } else if score >= 80 {
    println!("Grade: B");
    } else if score >= 70 {
    println!("Grade: C");
    } else {
    println!("Grade: F");
    }

    /*Using if as an Expression
    In Rust, if...else can also be used as an expression.

    This means you can assign the result of an if to a variable: */
    let time =20;
    let greeting = if time>15 {
        "good day."
    } else{
        "Good evening"
    };
    println!("{}",greeting);
    
    //simplified syntax
    let time2 = 20;
    let greeting = if time2 < 18 { "Good day." } else { "Good evening." };
    println!("{}", greeting);

    /*Don't Mix Types
    Note: The value from if and else must be the same type, like two pieces of text or two numbers (in the example above, both are strings).

    When you mix types, like a string and an integer, you'll get an error:

    Example */
    /*let number = 5;
    let result = if number < 10 { "Too small" } else { 100 };
    println!("{}", result); */
    /*Result:
    error[E0308]: `if` and `else` have incompatible types*/

    /*Match
    When you have many choices, using match is easier than writing lots of if...else.

    match is used to select one of many code blocks to be executed: */

    let day =4;

    match day {
        1 => println!("Monday"),
        2 => println!("tuesday"),
        3 => println!("wednesday"),
        4 => println!("thursday"),
        5 => println!("friday"),
        6 => println!("saturday"),
        7 => println!("sunday"),
        _ => println!("invalid day"),
    }

    //multiple matches
    let day2 = 6;

    match day2 {
        1|2|3|4|5 => println!("weekday"),
        6|7 => println!("weekend"),
        _ => println!("invalid input"),
    }

    //match with a Return Value
    //Just like if, match can also return a value:
    //This means you can save the result of a match into a variable:
    //Example
  let day3 = 4;

  let result = match day3 {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    4 => "Thursday",
    5 => "Friday",
    6 => "Saturday",
    7 => "Sunday",
    _ => "Invalid day.",
  };

    println!("{}", result);
