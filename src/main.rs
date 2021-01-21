use std::error::Error;
use std::io::{self, stdin, Write};
use std::{mem, process};

use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate colour;

mod controller;
mod ui;

//Global variables
static PATH: &str = "data/contactbook.csv";

#[derive(PartialEq)]
pub struct App {
    run: bool,
    uisearch: bool,
    contact_book: Vec<Contact>,
}

impl App {
    //Function to init app struct
    fn init(p_run: bool) -> Self {
        Self {
            run: p_run,
            uisearch: true,
            contact_book: vec![],
        }
    }

    // Function to import contact names from a csv file
    fn import_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rdr = Reader::from_path(PATH)?;

        println!("Reading and Importing CSV file.......\n");
        for result in rdr.deserialize() {
            let temp = result?;
            self.contact_book.push(temp);
        }
        Ok(())
    }

    // Function to export contact names from a csv file
    fn export_csv(&self) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(PATH)?;

        println!("Writing contacts to CSV file.......");
        for records in &self.contact_book {
            wtr.serialize(records)?;
        }
        wtr.flush()?;

        Ok(())
    }

    fn user_data_capture(&mut self) {
        let name = controller::input_capture("Enter First Name")
            .trim_end()
            .to_string();
        let surname = controller::input_capture("Enter Surname")
            .trim_end()
            .to_string();
        let dob = controller::input_capture("Enter date of birth")
            .trim_end()
            .to_string();
        let address = controller::input_capture("Enter your address")
            .trim_end()
            .to_string();
        let tel = controller::input_capture("Enter your telephone number")
            .trim_end()
            .to_string();
        let email = controller::input_capture("Enter email address")
            .trim_end()
            .to_string();

        //assigning captured data to struct and adding it to contact_book vector
        //let temp: Contact = Contact::new(name, surname, dob, address, tel, email);
        self.contact_book
            .push(Contact::new(name, surname, dob, address, tel, email));
    }

    //Function to print all the contacts on the vector to screen
    fn print_data(&self) {
        if self.contact_book.len() > 0 {
            for contact in &self.contact_book {
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

    //Function to print a struct Contact
    fn print_record(&self, record: &Contact) {
        println!("--------------------------------------------------------------");
        println!("{} {}", record.first_name, record.surname);
        println!("Tel: {} \tEmail: {}", record.tel, record.email);
        println!("Address: {}", record.address);
        println!("Date of Birth: {}", record.date_of_birth);
        println!("--------------------------------------------------------------");
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
}

fn init() -> App {
    let mut t: App = App::init(true);

    if let Err(err) = t.import_csv() {
        println!("error importing csv file: {}", err);
        process::exit(1);
    }

    t //return t:App
}

fn main() {
    // creating a mutable app struct and importing data from csv file into contact_book
    let mut app = init();

    while app.run == true {
        ui::main_ui();
        let mnu_choice = controller::input_capture("Enter Your Choice")
            .trim_end()
            .to_string();
        controller::main_match(mnu_choice, &mut app);
    }
}
