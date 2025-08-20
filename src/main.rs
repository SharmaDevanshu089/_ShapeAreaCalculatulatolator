use std::io;
//To Do Create a enum for getting shape sizes and then Find their areas
// enum Shape {
//     Circle(i32),//radius
//     Square(i32),//side in cm
//     Rectangle(i32,i32),//sideVertical sideHorizontal 
//     RightTriangle(i32,i32),//Side Side
//     Triangle(i32,i32,i32),
// }
//Dropped enum as its too long to do it this way
static pi:f32 = 3.14;
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
    match choice {
        1 => {
            println!("You have Chosen Circle.\n Please Enter the Radius:\n");
            let radius_of_circle = input_i32();
            let output = (2 as f32) * pi * (radius_of_circle as f32);
            print_output(output);
        },
        2 => {
            println!("You have Chosen Square.\n Please Enter the Length of Side");
            let side_of_square = input_i32();
            let output =( side_of_square*side_of_square) as f32;
            print_output(output);
        },
        3 => {
            println!("You Have Chosen Rectangle.\n Please Enter the Length:");
            let length_of_rectangle = input_i32();
            println!("Please enter width");
            let width_of_rectangle = input_i32();
            let output = (length_of_rectangle*width_of_rectangle) as f32;
            print_output(output);
        },
        4 => {
            println!("You Have Chosen Right Angled Triangle\n Please enter a side");
            let side_of_triangle = input_i32();
            println!("Please enter other side of triangle");
            let base_of_triangle = input_i32();
            let output = (((side_of_triangle*base_of_triangle) as f32) / 2 as f32) as f32;
            print_output(output);
        },
        5 => {
            println!("You Have Chosen Triangle \n Please tell all 1st Side:");
            let side1 = input_i32();
            println!("Please tell the Second side :");
            let side2 = input_i32();
            println!("Please tell the third side:");
            let side3 = input_i32();
            let semi_perimeter = (side1+side2+side3) as f32 / 2 as f32;
            let mut output = semi_perimeter*((semi_perimeter - side1 as f32)*(semi_perimeter - side2 as f32)* (semi_perimeter - side3 as f32));
            output = output.sqrt();
            print_output(output);
        },
        0 => {catch_error("RS");}
        _ => {get_choice();}
    }
}
fn catch_error(Error_Handle:&str){
    match Error_Handle {
        "WN" => {println!("You Should Enter Number near the Choice");get_choice();},
        "RS" => {initialise_cold_start();},
        _ => {println!("Unknown Error Occoured");initialise_cold_start();}
    }
}
fn initialise_cold_start(){
    println!("==================  Hello There  ==================\nThis is area Calculator V1 by Devanshu");
    println!("What Area would you like to find ? \n 1. Circle \n 2. Square \n 3. Rectangle \n 4. RightTriangle \n 5. Triangle");
    get_choice();
}
fn input_i32() -> i32 {
    let mut input_tmp = String::new();
    io::stdin().read_line(&mut input_tmp).unwrap_or_else(|_|{catch_error("WN");0});
    let final_input = input_tmp.trim().parse::<i32>();
    match final_input {
        Ok(final_input) => final_input,
        Err(final_input) => {catch_error("WN"); return 0;}
    }
}
fn print_output(output:f32){
    println!("The Area is {}",output);
    catch_error("RS");
    // match choice {
    //     1 => {
    //         println!("The Area is {}", output);
    //     },
    //     2 => {
    //         println!("The Area is {}",output);
    //     },
    //     3 => {
    //         println!("The Area is {}",output);
    //     },
    //     4 => {
    //         println!("The Area is {}",output);
    //     },
    //     5 => {
    //         println!("The Area is {}",output);
    //     },
    //     0 => {catch_error("RS");}
    //     _ => {get_choice();}
    // }
}