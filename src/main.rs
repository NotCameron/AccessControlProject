/* 
main.rs
Cameron Fisher
Section 1
Last Modified: 11/27/22
*/

// Import Modules
pub mod acm; //Access Control Matrix
use csv; //CSV parsing for creating test users
use std::{error::Error};

// Structs for Users and Admins
#[derive(Debug)]
struct User<> {
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    role: String,
}

#[derive(Debug)]
struct Admin<> {
    first_name: String,
    last_name: String,
    username: String,
    password: String,
}


// Main Function
fn main() -> Result<(), Box<dyn Error>>{
    // Create ACM
    let access_control = acm::gen_acm();

    // Login Prompt
    println!("1. Login as User");
    println!("2. Login as Admin");
    println!("3. Register User");
    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection);
    let selection = selection.trim();
    match selection {
        "1" => user_login().unwrap(),
        "2" => admin_login().unwrap(),
        _ => invalid_input().unwrap()
    };

    // Action Prompt
    println!("1. Login as User");
    println!("2. Login as Admin");
    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection);
    let selection = selection.trim();
    match selection {
        "1" => user_login().unwrap(),
        "2" => admin_login().unwrap(),
        _ => invalid_input().unwrap()
    };


    Ok(())
}

fn user_login() -> Result<(), Box<dyn Error>>{
    let mut logged_in = false;
    println!("Loading Users...");

    while !logged_in {
        // Enter Creds
        println!("--- Login ---");
        println!("Username: ");
        let mut user = String::new();
        std::io::stdin().read_line(&mut user).unwrap();
        let user = user.trim();

        println!("Password: ");
        let mut pass = String::new();
        std::io::stdin().read_line(&mut pass).unwrap();
        let pass = pass.trim();

        // CSV Reader reads csv file of users
        let mut reader = csv::Reader::from_path("./users.csv")?;
        let result = reader.records();
        
        // Iterate through CSV Results for match
        for record in result {
            let record = record?;
            // 3rd index (get(2)) and 4th index (get(3)) are username and password respectively. 
            // Need to unwrap() to get &str and trim() to pare down to the ASCII characters alone
            if user == String::from(record.get(2).unwrap().trim()) && pass == String::from(record.get(3).unwrap().trim()) {
                logged_in = true;
                break;
            }
        }

        // Check if user wants to try loggin in again. Mostly kept as a way of leaving without mashing ctrl + c all the time while testing
        if !logged_in {
            println!("Username or password was incorrect! \n Try again? (y/n)");
            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            let selection = selection.trim();
            if selection == "n"{
                break;
            }
        }
    }

    if logged_in {
        println!("Log in works!");
    }
    Ok(())
}

fn admin_login() -> Result<(), Box<dyn Error>>{
    println!("test");
    Ok(())
}

fn invalid_input() -> Result<(), Box<dyn Error>>{
    println!("Invalid input!");
    Ok(())
}

//     println!("Loading Admins...")
//     // Vector of admins
//     let mut admins = vec![];

//     // CSV Reader reads csv file of admins
//     let mut reader = csv::Reader::from_path("./admin.csv")?;
//     for result in reader.records(){
//         let record = result.expect("a CSV record");
//         println!("{:?}", record);
//     }

//     // Create struct of admin for every record and add to admins vector
//     for record in result {
//         let record = record?;
//         println!("{:?}", record);
//         let admin = Admin{first_name: String::from(record.get(0).unwrap()), 
//                     last_name: String::from(record.get(1).unwrap()), 
//                     username: String::from(record.get(2).unwrap()), 
//                     password: String::from(record.get(3).unwrap())};
//         admins.push(admin);
//     }

// Ok(())
// }
