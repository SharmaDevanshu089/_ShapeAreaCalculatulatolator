# 📐 Area Calculator in Rust

A simple, interactive command-line interface (CLI) application built in Rust for calculating the area of various geometric shapes.

---

## ✨ Features

* **Multiple Shapes**: Calculate the area for five different shapes:
    * Circle (using π ≈ 3.14)
    * Square
    * Rectangle
    * Right-Angled Triangle
    * General Triangle (using Heron's formula)
* **Interactive Menu**: A user-friendly menu to select the desired shape.
* **Robust Input Handling**: Basic error handling is in place to manage non-numeric inputs, prompting the user to try again.
* **Continuous Operation**: The application loops after each calculation, allowing for multiple calculations without restarting.
* **Zero Dependencies**: Written in pure Rust with no external crates.

---

## 🚀 Getting Started

### Prerequisites

To compile and run this project, you need to have the **Rust toolchain** installed on your machine. If you don't have it, you can install it from the official [Rust website](https://www.rust-lang.org/tools/install).

### Installation & Execution

1.  **Clone the repository** to your local machine:
    ```bash
    git clone <your-repository-url>
    ```

2.  **Navigate to the project directory**:
    ```bash
    cd <your-project-directory>
    ```

3.  **Compile and run the application** using Cargo:
    ```bash
    cargo run
    ```
    The program will start, and you can follow the on-screen instructions.

---

## 💻 How to Use

Once the program is running, you'll see a welcome message and a menu of shapes.

```
==================  Hello There  ==================
This is area Calculator V1 by Devanshu
What Area would you like to find ?
 1. Circle
 2. Square
 3. Rectangle
 4. RightTriangle
 5. Triangle
```

1.  Enter the number corresponding to the shape of your choice (e.g., `1` for Circle).
2.  The program will then prompt you to enter the required dimensions (like radius, side length, etc.).
3.  After you provide the dimensions, it will print the calculated area.
4.  The menu will then be displayed again for a new calculation. To exit, simply enter `0`.

### Example Session

```
Please enter your Choice Below:
> 3
You Have Chosen Rectangle.
 Please Enter the Length:
> 10
Please enter width
> 5
The Area is 50
==================  Hello There  ==================
...
```

---

## 📂 Code Structure Overview

The program's logic is contained within `main.rs` and is organized into several key functions:

* `main()`: The entry point of the application. It simply calls `initialise_cold_start()`.
* `initialise_cold_start()`: Prints the welcome banner and the main menu of shape options. It then calls `get_choice()` to start the process.
* `get_choice()`: Reads the user's menu selection from the command line and passes it to `execute_choice()`. It also handles parsing errors if the input is not a valid number.
* `execute_choice(choice: i32)`: Contains the core logic. It uses a `match` statement to determine which shape's area to calculate based on the user's choice.
* `input_i32() -> i32`: A helper function designed to reliably read and parse an integer (`i32`) from user input.
* `print_output(output: f32)`: Displays the final calculated area to the user.
* `catch_error(error_handle: &str)`: A simple function for handling predefined errors, such as wrong numeric input (`"WN"`) or restarting the program (`"RS"`).

