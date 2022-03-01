//! A simple CLI based bill manager.

use ::std::collections::HashMap;
use bill_minder::{*, sql};


fn main() {
    let mut all_bills = HashMap::new();

    match sql::create_db() {
        Ok(i) => i,
        Err(e) => println!("Error creating DB: {}", e)
    }
    match sql::load_db(&mut all_bills) {
        Err(e) => println!("Error loading DB: {}", e),
        Ok(ok) => ok
    }

    menu(&mut all_bills);
    match sql::save_db(&mut all_bills) {
        Err(e) => println!("Error saving DB: {}", e),
        Ok(ok) => ok
    }
}

/// Main menu and loop for the app.
fn menu(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    loop {
        println!("______________________");
        println!("<< Bill Minder >>");
        println!("Total Bills: {}\n", all_bills.len());
        println!("1) Add Bill");
        println!("2) View Bills");
        println!("3) Remove Bill");
        println!("4) Edit Bill");
        println!("0) Quit");
        println!("______________________");
        println!("Select an option\nthen ENTER: ");
        let choice = get_user_input();
        let choice = choice.trim();
        match choice {
            "0" => break,
            "1" => add_bill(all_bills),
            "2" => view_bills(all_bills),
            "3" => remove_bill(all_bills),
            "4" => edit_bill(all_bills),
            _x => {
                clear_scrn();
                continue;
            }
        }
    }
}


fn add_bill(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    println!("<Add> Enter bill name:");
    let name = get_user_input();
    let name = name.trim().to_uppercase().to_string();
    if name.len() < 1 {
        println!("Name must have at least one character, try again.");
        return;
    }

    println!("<Add> Enter bill amount:");
    let amount = get_user_input();
    let amount = amount.trim().parse::<f64>();
    let amount = match amount {
        Ok(f) => f,
        Err(_) => {
            println!("Amount must be a dollar amount, try again.");
            return;
        }
    };
    all_bills.insert(name, amount);
    clear_scrn();
}

fn view_bills(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    println!("______________________");
    println!("Bill Summary");
    if all_bills.len() < 1 {
        println!("All bills paid!!!");
        return;
    } else {
        for (k, v) in all_bills {
            println!("{}, Amount:{},", k, v);
        }
    }
}

fn remove_bill(all_bills: &mut HashMap<String, f64>) {
    clear_scrn();
    view_bills(&mut all_bills.clone());
    if all_bills.len() < 1 {
        println!("No bills to remove! Great job!");
        return;
    }


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
    if all_bills.len() < 1 {
        println!("No bills to edit! Great job!");
        return;
    }

    let (answer, name) = select_bill(all_bills);
    if answer == "y" {
        println!("Enter the new amount: ");
        let new_amount = get_user_input();

        match new_amount.trim().parse::<f64>() {
            Ok(f) => {
                let new_amount = f;
                all_bills.insert(name.clone(), new_amount);
                println!("{name} updated to {new_amount}");
            }
            Err(_) => {
                println!(
                    "Could not convert input to dollar amount, try again.", );
                ()
            }
        }
    } else {
        clear_scrn();
        println!("<Remove> Try again...");
    }
}

