use std::error::Error;
use std::io::{self, stdin, Write};
use std::{mem, process};

use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate colour;

//Global variables
static PATH: &str = "data/contactbook.csv";

struct App {
    run: bool,
    contact_book: Vec<Contact>,
}

impl App {
    fn new(run: bool) -> Self {
        Self {
            run: run,
            contact_book: vec![],
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Contact {
    first_name: String,
    surname: String,
    date_of_birth: String,
    address: String,
    tel: String,
    email: String,
}

#[allow(dead_code)]
impl Contact {
    fn new(
        first_name: String,
        surname: String,
        date_of_birth: String,
        address: String,
        tel: String,
        email: String,
    ) -> Self {
        Self {
            first_name: first_name,
            surname: surname,
            date_of_birth: date_of_birth,
            address: address,
            tel: tel,
            email: email,
        }
    }

    fn empty() -> Self {
        Self {
            first_name: "".to_string(),
            surname: "".to_string(),
            date_of_birth: "".to_string(),
            address: "".to_string(),
            tel: "".to_string(),
            email: "".to_string(),
        }
    }

    //Function to print all the contacts on the vector to screen
    fn print_data(&self, contact_book: &Vec<Contact>) {
        if contact_book.len() > 0 {
            for contact in contact_book {
                println!("--------------------------------------------------------------");
                println!("{} {}", contact.first_name, contact.surname);
                println!("Tel: {} \tEmail: {}", contact.tel, contact.email);
                println!("Address: {}", contact.address);
                println!("Date of Birth: {}", contact.date_of_birth);
                println!("--------------------------------------------------------------");
            }
        } else {
            println!("Contact Book is empty");
        }
    }

    // function to print only one record from struct
    fn print_record(&self, record: &Contact) {
        println!("--------------------------------------------------------------");
        println!("{} {}", record.first_name, record.surname);
        println!("Tel: {} \tEmail: {}", record.tel, record.email);
        println!("Address: {}", record.address);
        println!("Date of Birth: {}", record.date_of_birth);
        println!("--------------------------------------------------------------");
    }

    fn remove_data(&self, contact_book: &mut Vec<Contact>, p_fname: &str, p_sname: &str) {
        let mut index: usize = 0;

        println!("Searching contacts...");
        for record in contact_book.into_iter() {
            if (record.first_name == p_fname) && (record.surname == p_sname) {
                contact_book.remove(index);
                break;
            }
            index += 1;
        }
    }

    fn edit_data(&self, contact_book: &mut Vec<Contact>, fname: &str, sname: &str) {
        let mut index: usize = 0;

        println!("Searching contacts...");
        for record in contact_book {
            if record.first_name == fname && record.surname == sname {
                record.tel = "222-111-1111".to_string();
                break;
            }
            index += 1;
        }
    }

    // Function to import contact names from a csv file
    fn import_csv(&self, contact_book: &mut Vec<Contact>) -> Result<(), Box<dyn Error>> {
        let mut rdr = Reader::from_path(PATH)?;

        println!("Reading and Importing CSV file.......\n");
        for result in rdr.deserialize() {
            let temp = result?;
            contact_book.push(temp);
        }
        Ok(())
    }

    fn export_csv(&self, contact_book: &Vec<Contact>) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(PATH)?;

        println!("Writing contacts to CSV file.......");
        for records in contact_book {
            wtr.serialize(records)?;
        }
        wtr.flush()?;

        Ok(())
    }

    fn search_contact(&self, contact_book: &Vec<Contact>, text: &str) {
        println!("Searching contacts...");
        for record in contact_book {
            if record.first_name == text {
                self.print_record(record);
            }
        }
    }
}

//Functions calls input_capture and stores into struct
fn user_data_capture(contact_book: &mut Vec<Contact>) {
    let name = input_capture("Enter First Name").trim_end().to_string();
    let surname = input_capture("Enter Surname").trim_end().to_string();
    let dob = input_capture("Enter date of birth").trim_end().to_string();
    let address = input_capture("Enter your address").trim_end().to_string();
    let tel = input_capture("Enter your telephone number")
        .trim_end()
        .to_string();
    let email = input_capture("Enter email address").trim_end().to_string();

    //assigning captured data to struct and adding it to contact_book vector
    let temp: Contact = Contact::new(name, surname, dob, address, tel, email);
    contact_book.push(temp);
}

// Function for getting user input and returns the input
fn input_capture(text: &str) -> String {
    let mut input = String::new();

    println!("{}: ", text);
    io::stdout().flush().unwrap();

    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    return input;
}

//TODO function for search Function
// Menu for searching by content(HashMaps Keys)
fn search_ui() {}

//Function that has the main menu
fn main_ui() {
    //TODO make main menu for contact book.
    // Add, Edit, Delete, Show, Search, Save Contact Book, Load Contact Book
    cyan_ln!("-------------Contact Address Book--------------");
    blue_ln!("Designed in Rust Language \t By Peter Keller");
    cyan_ln!("-----------------------------------------------");
    println!();
    white!("1) ");
    yellow_ln!("Add New Contact");
    white!("2) ");
    yellow_ln!("Edit Contact");
    white!("3) ");
    yellow_ln!("Delete Contact");
    white!("4) ");
    yellow_ln!("Search For Contact");
    white!("5) ");
    yellow_ln!("Show All Contacts");
    white!("6) ");
    yellow_ln!("Quit");
    cyan_ln!("-----------------------------------------------");
}

fn init() -> (App, Contact) {
    let mut app = App::new(true);
    let mut c = Contact::empty();

    //import csv into cb
    if let Err(err) = c.import_csv(&mut app.contact_book) {
        println!("error running example: {}", err);
        process::exit(1);
    }

    return (app, c);
}

fn main() {
    // run init() which returns struct Contact and Vec<Contact> with imported csv file
    let (mut app, mut contact) = init();

    while (app.run == true) {
        main_ui();
        let mnu_choice = input_capture("Enter Your Choice").trim_end().to_string();

        match mnu_choice.as_str() {
            "1" => println!("Add New Contact"),
            "2" => println!("Edit Contact"),
            "3" => {
                println!("Delete Contact")
            }
            "4" => {
                println!("Search Contact");
            }
            "5" => {
                println!("Show All Contact");
                contact.print_data(&app.contact_book);
            }
            "6" => {
                println!("Quit");
                break;
            }
            _ => println!("Please pick a number from the menu"),
        }
    }

    //export csv
    if let Err(err) = contact.export_csv(&app.contact_book) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
