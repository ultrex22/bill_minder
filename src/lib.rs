

use ::std::collections::HashMap;
use clearscreen::ClearScreen;
use std::io;


/// Helper function for selecting and confirming choice of a bill to work with.
pub fn select_bill(all_bills: &mut HashMap<String, f64>) -> (String, String) {
    println!("\nSelect the bill you need: ");
    let bill_to_edit = get_user_input();
    let bill_to_edit = bill_to_edit.trim().to_uppercase().to_string();
    let clone_all_bills = &mut all_bills.clone();
    match clone_all_bills.get_key_value(&bill_to_edit) {
        Some(s) => {
            let (name, price) = s;
            println!("Is this the bill? (y/n):  Bill: {}, {}", name, price);
            let answer = get_user_input();
            let answer = answer.trim().to_lowercase().to_string();
            (answer, name.clone())
        }
        None => return ("n".to_string(), "None".to_string()),
    }
}

pub fn clear_scrn() {
    ClearScreen::default()
        .clear()
        .expect("Screen clearing failed.");
}

/// Helper function to get user input.
pub fn get_user_input() -> String {
    let mut user_input = String::new();
    let _read_input = io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input.");
    user_input
}
