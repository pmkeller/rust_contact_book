use std::collections::HashMap;
use std::io::{stdin, self, Write};

#[derive(Eq, PartialEq, Debug, Default)]
struct Contact {
    first_name: String,
    surname: String,
    date_of_birth: String,
    address: String,
    tel: String,
    email: String,
    contact_book: Vec<HashMap<String,String>>,
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
            contact_book: vec![]
        }
    }

    //Function used to push struct data into struct vector of hashmaps contact_book
    fn data_push(&mut self) {
        let mut contact_hash = HashMap::new();

        contact_hash.insert("Name".to_string(), self.first_name.trim_end().to_string());
        contact_hash.insert("Surname".to_string(), self.surname.trim_end().to_string());
        contact_hash.insert("DateofBirth".to_string(), self.date_of_birth.trim_end().to_string());
        contact_hash.insert("Address".to_string(), self.address.trim_end().to_string());
        contact_hash.insert("Tel".to_string(), self.tel.trim_end().to_string());
        contact_hash.insert("Email".to_string(), self.email.trim_end().to_string());

        self.contact_book.push(contact_hash);
    }

    //Function to print contact book to screen
    fn print_data(&self) {
        if self.contact_book.len() > 0 {
            for contact_hash in &self.contact_book {
                println!("--------------------------------------------------------------");
                println!("{} {}", contact_hash.get("Name").unwrap(), contact_hash.get("Surname").unwrap());
                println!("Tel: {} \tEmail: {}", contact_hash.get("Tel").unwrap(), contact_hash.get("Email").unwrap());
                println!("Address: {}", contact_hash.get("Address").unwrap());
                println!("Date of Birth: {}", contact_hash.get("DateofBirth").unwrap());
                println!("--------------------------------------------------------------");
            }
        } else {
            println!("Contact Book is empty");
        }
    }

    //TODO remove data based on key search from vector
    fn remove_data() {}

    //TODO edit data based on key search from vector
    fn edit_data() {}

    //TODO Import CSV Function
// Function to import contact names from a csv file
    fn import_csv() {
        //check crates for a csv loader
    }

    //TODO Export CSV Function
// Function to export contact vector to a csv file
    fn export_csv() {
    }

    //TODO Search Function
// iterate through
    fn search_contact() {
    }

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

//Functions calls input_capture and stores into struct
fn  user_data_capture(p_contact: &mut Contact) {
    
    let name = input_capture("Enter First Name");
    let surname = input_capture("Enter Surname");
    let dob = input_capture("Enter date of birth");
    let address = input_capture("Enter your address");
    let tel = input_capture("Enter your telephone number");
    let email = input_capture("Enter email address");

    p_contact.first_name = name;
    p_contact.surname = surname;
    p_contact.date_of_birth = dob;
    p_contact.address = address;
    p_contact.tel = tel;
    p_contact.email = email;

    p_contact.data_push()
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

    let mut contact = Contact{..Default::default()};

    user_data_capture(&mut contact);
    contact.print_data();
}
