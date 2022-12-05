/* 
main.rs
Cameron Fisher
Section 1
Last Modified: 12/04/22
*/

// Import Modules
pub mod acm; // Access Control Matrix
pub mod login; // Login Functions
use std::{error::Error};


// Main Function
fn main() -> Result<(), Box<dyn Error>>{

    // Login Prompt
    let mut logged_in = false;
    let mut user_logged_in = String::from("0");
    let mut admin_logged_in = false;
    while !logged_in {
        println!("1. Login as User");
        println!("2. Login as Admin");
        println!("3. Register User");
        println!("4. Close program");
        let mut selection = String::new();
        std::io::stdin().read_line(&mut selection);
        let selection = selection.trim();
        match selection {
            "1" => user_logged_in = login::user_login(),
            "2" => admin_logged_in = login::admin_login(),
            "4" => break,
            _ => invalid_input().unwrap()
        };
        if (user_logged_in != String::from("0")) || admin_logged_in {
            logged_in=true
        }
    }

    // User Actions Prompt
    loop {
        if user_logged_in != "0" {
            // Create ACM
            // Configure default in acm.rs
            let access_control = acm::gen_acm();
            let role = &user_logged_in.to_ascii_lowercase(); // Send into function for permission cehcking
            
            // Menu
            println!("1. Submit manuscript");
            println!("2. Invite associate editors");
            println!("3. Invite reviewers");
            println!("4. Submit review");
            println!("5. Make recommendation");
            println!("6. Close program");
            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection);
            let selection = selection.trim();
            match selection {
                "1" => acm::submit_manuscript(access_control, role),
                "2" => acm::invite_editors(access_control, role),
                "3" => acm::invite_reviewers(access_control, role),
                "4" => acm::submit_review(access_control, role),
                "5" => acm::make_recommendation(access_control, role),
                "6" => break,
                _ => invalid_input().unwrap()
            };
        } else {
            break;
        }
    }

    // Admin Actions Prompt
    loop {
        if admin_logged_in {
            // Create ACM
            // Configure default in acm.rs
            let access_control = acm::gen_acm();
            let role = &user_logged_in.to_ascii_lowercase();
            
            // Menu
            println!("1. Print Access Control Matrix");
            println!("2. Close program");
            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection);
            let selection = selection.trim();
            match selection {
                "1" => acm::print_acm(access_control),
                "2" => break,
                _ => invalid_input().unwrap()
            };
        } else {
            break;
        }
    }

    Ok(())
}


fn invalid_input() -> Result<(), Box<dyn Error>>{
    println!("Invalid input!");
    Ok(())
}


