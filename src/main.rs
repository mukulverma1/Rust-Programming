// Restaurant Ordering System

use std::io;

enum Menu {
    Pizza { size: String, crust: String },
    Burger { with_cheese: bool },
    Drink { size: String },
}

fn calculate_price(item: Menu) -> f64 {
    match item {
        Menu::Pizza { size, crust } => {
            let mut price = 8.0;
            if size == "medium" {
                price += 2.0;
            } else if size == "large" {
                price += 4.0;
            }
            if crust == "stuffed" {
                price += 1.0;
            }
            price
        }
        Menu::Burger { with_cheese } => {
            let mut price = 5.0;
            if with_cheese {
                price += 2.0;
            }
            price
        }
        Menu::Drink { size } => {
            let mut price = 2.0;
            if size == "medium" {
                price += 0.5;
            } else if size == "large" {
                price += 1.0;
            }
            price
        }
    }
}

fn main() {
    loop {
        println!("Welcome to the Restaurant Ordering System");
        println!("1. Order Pizza");
        println!("2. Order Burger");
        println!("3. Order Drink");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");

        match choice {
            1 => {
                let mut size = String::new();
                let mut crust = String::new();

                println!("Enter pizza size (small, medium, large): ");
                io::stdin().read_line(&mut size).expect("Failed to read line");
                let size = size.trim().to_string();

                println!("Enter crust type (regular, stuffed): ");
                io::stdin().read_line(&mut crust).expect("Failed to read line");
                let crust = crust.trim().to_string();

                let pizza = Menu::Pizza { size, crust };
                let price = calculate_price(pizza);
                println!("The price of your pizza is: ${:.2}", price);
            }
            2 => {
                let mut with_cheese = String::new();

                println!("Do you want cheese? (yes/no): ");
                io::stdin().read_line(&mut with_cheese).expect("Failed to read line");
                let with_cheese = with_cheese.trim().to_lowercase() == "yes";

                let burger = Menu::Burger { with_cheese };
                let price = calculate_price(burger);
                println!("The price of your burger is: ${:.2}", price);
            }
            3 => {
                let mut size = String::new();

                println!("Enter drink size (small, medium, large): ");
                io::stdin().read_line(&mut size).expect("Failed to read line");
                let size = size.trim().to_string();

                let drink = Menu::Drink { size };
                let price = calculate_price(drink);
                println!("The price of your drink is: ${:.2}", price);
            }
            4 => {
                println!("Thank you for visiting!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
