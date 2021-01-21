use crate::{ui, App, Contact};
use std::io::{self, stdin, Write};
use std::process;

// Function for getting user input and returns the input
pub fn input_capture(text: &str) -> String {
    let mut input = String::new();

    println!("{}: ", text);
    io::stdout().flush().unwrap();

    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    return input;
}

pub fn main_match(mnu_choice: String, app: &mut App) {
    match mnu_choice.as_str() {
        "1" => {
            println!("Add New Contact");
            app.user_data_capture();
        }
        "2" => println!("Edit Contact"),
        "3" => {
            println!("Delete Contact")
        }
        "4" => {
            println!("Search Contact");
            app.uisearch = true;

            while app.uisearch == true {
                ui::search_ui();

                search_match(app);
            }
        }
        "5" => {
            println!("Show All Contact");
            app.print_data();
        }
        "6" => {
            println!("Quit");
            app.run = false;

            //export csv
            if let Err(err) = app.export_csv() {
                println!("error running example: {}", err);
                process::exit(1);
            }
        }
        _ => println!("Please pick a number from the menu"),
    }
}

pub fn search_match(app: &mut App) {
    let choice = input_capture("Enter Your Choice").trim_end().to_string();
    match choice.as_str() {
        "1" => {
            searchby_name(&app);
        }
        "2" => {
            searchby_surname(&app);
        }
        "3" => {
            searchby_both(&app);
        }
        "4" => {
            let tel = input_capture("Please enter telephone number");
            searchby_tel(tel, &app);
        }
        "5" => {
            let email = input_capture("Please enter email address");
            searchby_email(email, &app);
        }
        "6" => {
            app.uisearch = false;
        }
        _ => {
            println!("Please pick a number from the menu");
        }
    }
}

pub fn searchby_name(app: &App) {
    let fname = input_capture("Please enter first name");

    println!("Searching contacts...");
    for record in &app.contact_book {
        if record.first_name == fname.trim().to_string() {
            app.print_record(record);
        }
    }
}

pub fn searchby_surname(app: &App) {
    let sname = input_capture("Please enter first name");

    println!("Searching contacts...");
    for record in &app.contact_book {
        if record.surname == sname.trim().to_string() {
            app.print_record(record);
        }
    }
}

pub fn searchby_both(app: &App) {
    let fname = input_capture("Please enter first name");
    let sname = input_capture("Please enter first name");

    println!("Searching contacts...");
    for record in &app.contact_book {
        if record.first_name == fname.trim().to_string()
            && record.surname == sname.trim().to_string()
        {
            app.print_record(record);
        }
    }
}

pub fn searchby_tel(tel: String, app: &App) {
    println!("Searching contacts...");
    for record in &app.contact_book {
        if record.tel == tel.trim().to_string() {
            app.print_record(record);
        }
    }
}

pub fn searchby_email(email: String, app: &App) {
    println!("Searching contacts...");
    for record in &app.contact_book {
        if record.email == email.trim().to_string() {
            app.print_record(record);
        }
    }
}

pub fn edit_contact(app: &mut App) {
    let fname = input_capture("Please enter first name");
    let sname = input_capture("Please enter first name");
}
