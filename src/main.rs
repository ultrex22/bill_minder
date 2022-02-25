// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use ::std::collections::HashMap;
use clearscreen::ClearScreen;
use std::io;

fn main() {
    menu();
}

fn menu() {
    let mut all_bills = HashMap::new();
    clear_scrn();
    loop {
        println!("______________________");
        println!("Main Menu\n");
        println!("Total Bills: {}", all_bills.len());
        println!("1) Add Bill");
        println!("2) View Bills");
        println!("3) Remove Bill");
        println!("4) Edit Bill");
        println!("0) Quit");
        println!("______________________");
        println!("Select an option: ");
        let choice = get_user_input();
        let choice = choice.trim();
        match choice {
            "0" => break,
            "1" => add_bill(&mut all_bills),
            "2" => view_bills(&mut all_bills),
            "3" => remove_bill(&mut all_bills),
            "4" => edit_bill(&mut all_bills),
            _x => {
                clear_scrn();
                continue;
            }
        }
    }
}

fn clear_scrn() {
    ClearScreen::default()
        .clear()
        .expect("Screen clearing failed.");
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    let _read_input = io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input
}

fn add_bill(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    println!("<Add> Enter bill name:");
    let name = get_user_input();
    let name = name.trim().to_uppercase().to_string();
    println!("<Add> Enter bill amount:");

    let amount = get_user_input();
    let amount = amount.trim().parse::<f64>().unwrap();
    // println!("Name:{} Amount:{:?}", name, amount);
    all_bills.insert(name, amount);
    clear_scrn();
}

fn view_bills(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    println!("______________________");
    println!("Bill Summary");
    for (k, v) in all_bills {
        println!("{}, Amount:{},", k, v);
    }
}

fn remove_bill(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    view_bills(&mut all_bills.clone());
    let (answer, name) = select_bill(all_bills);
    if answer == "y" {
        all_bills.remove(&name);
        clear_scrn();
        println!("\n{:?} , Removed!", name);
    } else {
        clear_scrn();
        println!("<Remove> Try again...");
    }
}

fn edit_bill(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    view_bills(all_bills);
    let (answer, name) = select_bill(all_bills);
    if answer == "y" {
        println!("Enter the new amount: ");
        let new_amount = get_user_input();
        let new_amount = new_amount.trim().parse::<f64>().unwrap();
        all_bills.insert(name.clone(), new_amount);
        println!("{name} updated to {new_amount}");
    } else {
        clear_scrn();
        println!("<Remove> Try again...");
    }
}

fn select_bill(all_bills: &mut HashMap<String, f64>) -> (String, String) {
    println!("\nSelect the bill you need: ");
    let bill_to_edit = get_user_input();
    let bill_to_edit = bill_to_edit.trim().to_uppercase().to_string();
    let clone_all_bills = &mut all_bills.clone();
    let (name, price) = clone_all_bills.get_key_value(&bill_to_edit).unwrap();
    println!("Is this the bill? (y/n):  Bill: {}, {}", name, price);
    let answer = get_user_input();
    let answer = answer.trim().to_lowercase().to_string();
    (answer, name.clone())
}
