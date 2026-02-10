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
