use std::io::{stdin, self, Write};
use std::error::Error;
use std::{process, mem};

use csv::{Reader, Writer};
use serde::{Serialize, Deserialize};

//Global variables
static PATH: &str = "data/contactbook.csv";

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Contact {
    first_name: String,
    surname: String,
    date_of_birth: String,
    address: String,
    tel: String,
    email: String
}

#[allow(dead_code)]
impl Contact {
    fn new(first_name: String, surname: String, date_of_birth: String, address: String, tel: String, email: String) -> Self {
        Self {
            first_name: first_name,
            surname: surname,
            date_of_birth: date_of_birth,
            address: address,
            tel: tel,
            email: email,
        }
    }

    //Function to print contact book to screen
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

    fn remove_data(&self, contact_book: &mut Vec<Contact>, p_fname: &str, p_sname: &str) {

        let mut index: usize = 0;

        println!("Searching contacts...");
        for record in contact_book.clone() {
            if (record.first_name == p_fname) && (record.surname ==  p_sname) {
                contact_book.remove(index);
            }
            index += 1;
        }

    }

    //TODO edit data based on key search from vector
    fn edit_data(&self) {
    }

    // Function to import contact names from a csv file
    fn import_csv(&self, contact_book: &mut Vec<Contact>) -> Result<(), Box<dyn Error>>{

        let mut rdr = Reader::from_path(PATH)?;

        println!("Reading and Importing CSV file.......");
        for result in rdr.deserialize() {
            let temp= result?;

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

        let mut found: Vec<Contact> = Vec::new();

        println!("Searching contacts...");
        for record in contact_book {
            if record.first_name == text {
                found.push(record.clone());
            }
        }

        self.print_data(&found);
    }

}

//Functions calls input_capture and stores into struct
fn  user_data_capture(contact_book: &mut Vec<Contact>) {

    let name = input_capture("Enter First Name").trim_end().to_string();
    let surname = input_capture("Enter Surname").trim_end().to_string();
    let dob = input_capture("Enter date of birth").trim_end().to_string();
    let address = input_capture("Enter your address").trim_end().to_string();
    let tel = input_capture("Enter your telephone number").trim_end().to_string();
    let email = input_capture("Enter email address").trim_end().to_string();

    //assigning captured data to struct and adding it to contact_book vector
    let temp:Contact= Contact::new(name, surname, dob, address, tel, email);
    contact_book.push(temp);
}


// Function for getting user input and returns the input
fn input_capture(text: &str) -> String {
    let mut input = String::new();

    println!("{}: ", text);
    io::stdout().flush().unwrap();

    stdin().read_line(&mut input)
        .expect("Did not enter a correct string");

    return input;
}


//TODO function for search Function
// Menu for searching by content(HashMaps Keys)
fn search_ui(){
}

//Function that has the main menu
fn main_ui(){
    //TODO make main menu for contact book.
    // Add, Edit, Delete, Show, Search, Save Contact Book, Load Contact Book
}


fn main() {

    // Declaring a new variable of struct Contact
    let mut contact = Contact{..Default::default()};
    //this vector stores the data of contact struct as a vector of Contact
    let mut contact_book: Vec<Contact> = Vec::new();

   //import csv
    if let Err(err) = contact.import_csv(&mut contact_book) {
        println!("error running example: {}", err);
        process::exit(1);
    }

    //Get new contact data from user
    //user_data_capture(&mut contact_book);

    //run search implementation
    //contact.search_contact(&contact_book,"peter");

    //run delete contact from contact_book
    //contact.remove_data(&mut contact_book, "peter", "keller");

    //print current contact
    //contact.print_data(&contact_book);

    //export csv
    if let Err(err) = contact.export_csv(&contact_book) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
