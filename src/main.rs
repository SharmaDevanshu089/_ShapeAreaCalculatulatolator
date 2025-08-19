use std::io;
//To Do Create a enum for getting shape sizes and then Find their areas
enum Shape {
    Circle(i32),//radius
    Square(i32),//side in cm
    Rectangle(i32,i32),//sideVertical sideHorizontal 
    RightTriangle(i32,i32),//Side Side
    Triangle(i32,i32,i32),
}
fn main() {
    initialise_cold_start();

}
fn get_choice() -> (){
    println!("Please enter your Choice Below:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice);
    let ans = choice.trim().parse::<i32>();
    match ans {
        Ok(ans) => execute_choice(ans),
        Err(ans)=> catch_error("WN")
    }
}
fn execute_choice(choice:i32) {
    println!("{}",choice);
}
fn catch_error(Error_Handle:&str){
    match Error_Handle {
        "WN" => {println!("You Should Enter Number");get_choice();},
        _ => {println!("Unknown Error Occoured");initialise_cold_start();}
    }
}
fn initialise_cold_start(){
    println!("==================  Hello There  ==================\nThis is area Calculator V1 by Devanshu");
    println!("What Area would you like to find ? \n 1. Circle \n 2. Square \n 3. Rectangle \n 4. RightTriangle \n 5. Triangle");
}