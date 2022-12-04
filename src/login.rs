/* 
login.rs
Cameron Fisher
Section 1
Last Modified: 12/04/22
*/

use csv; //CSV parsing for creating test users



// Change to return a Result at some point. Seems to be best practice and allows for error handling
pub fn user_login() -> String {
    let mut logged_in = false;
    let mut role = String::from("0");
    println!("Loading Users...");

    while !logged_in {
        // Enter Creds
        println!("--- Login ---");
        println!("Username: ");
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).unwrap();
        let username = username.trim();

        println!("Password: ");
        let mut pass = String::new();
        std::io::stdin().read_line(&mut pass).unwrap();
        let pass = pass.trim();

        // CSV Reader reads csv file of users
        let mut reader = csv::Reader::from_path("./users.csv").unwrap();
        let result = reader.records();

        // Iterate through CSV Results for match
        for record in result {
            let record = record.unwrap();
            // 3rd index (get(2)) and 4th index (get(3)) are username and password respectively. 
            // Need to unwrap() to get &str and trim() to pare down to the ASCII characters alone
            if username == String::from(record.get(2).unwrap().trim()) && pass == String::from(record.get(3).unwrap().trim()) {
                logged_in = true;
                role = String::from(record.get(4).unwrap().trim());
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
        return role;
    }

    return String::from("0")
}

pub fn admin_login() -> bool {
    println!("Loading Admins...");
    let mut logged_in = false;

    while !logged_in {
        // Enter Creds
        println!("--- Login ---");
        println!("Username: ");
        let mut admin = String::new();
        std::io::stdin().read_line(&mut admin).unwrap();
        let admin = admin.trim();

        println!("Password: ");
        let mut pass = String::new();
        std::io::stdin().read_line(&mut pass).unwrap();
        let pass = pass.trim();

        // CSV Reader reads csv file of users
        let mut reader = csv::Reader::from_path("./admin.csv").unwrap();
        let result = reader.records();
        
        // Iterate through CSV Results for match
        for record in result {
            let record = record.unwrap();
            // 3rd index (get(2)) and 4th index (get(3)) are username and password respectively. 
            // Need to unwrap() to get &str and trim() to pare down to the ASCII characters alone
            if admin == String::from(record.get(2).unwrap().trim()) && pass == String::from(record.get(3).unwrap().trim()) {
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
        return true;
    }

    false
}