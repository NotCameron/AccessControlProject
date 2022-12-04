/* 
acm.rs
Cameron Fisher
Section 1
Last Modified: 12/04/22

The access control matrix module
*/

// Imports
use std::collections::HashMap;

// ACM Structure
pub struct ACM{
    // Roles Array (Row, 2nd num) in order:
    // author, editor, associate_editor, reviewer
    // Actions/Permissions (Column, 1st num) array in order:
    // submit, invite_associates, invite_reviewers, submit_review, make_recommendation
    acm: [[bool; 5]; 4],
    roles: HashMap<String, usize>,
}

// Generates ACM for main.rs such that it can be passed to the action functions
pub fn gen_acm() -> ACM {
    // Actions/Permissions array in order:
    // submit, invite_associates, invite_reviewers, submit_review, make_recommendation

    // Author
    let author_actions = [true, false, false, false, false];

    // Editor
    let editor_actions = [false, true, false, false, false];

    // Associate Editor
    let associate_editor_actions = [false, false, true, true, true];

    // Reviewer
    let reviewer_actions = [false, false, false, true, true];

    // Compile permissions into 2D matrix
    let ac_matrix = [author_actions, editor_actions, associate_editor_actions, reviewer_actions];

    // Create HashMap of roles
    let mut roles: HashMap<String, usize>  = HashMap::new();
    roles.insert(String::from("author"), 1);
    roles.insert(String::from("editor"),2 );
    roles.insert(String::from("associate editor"), 3);
    roles.insert(String::from("reviewer"), 4);

    // Create ACM struct
    let acm = ACM {acm: ac_matrix, roles: roles};

    acm
}

// For admin printing ACM (also for debugging)
pub fn print_acm() {
    println!("__________________________________________________________________________________________________________________________");
    println!("|       Role       | Submit Manuscript | Invite Associate Editor | Invite Reviewer | Submit Review | Make Recommendation |");
    println!("__________________________________________________________________________________________________________________________");
    println!("|      Author      | Submit Manuscript | Invite Associate Editor | Invite Reviewer | Submit Review | Make Recommendation |");
    println!("__________________________________________________________________________________________________________________________");
    println!("|      Editor      | Submit Manuscript | Invite Associate Editor | Invite Reviewer | Submit Review | Make Recommendation |");
    println!("__________________________________________________________________________________________________________________________");
    println!("| Associate Editor | Submit Manuscript | Invite Associate Editor | Invite Reviewer | Submit Review | Make Recommendation |");
    println!("__________________________________________________________________________________________________________________________");
    println!("|     Reviewer     | Submit Manuscript | Invite Associate Editor | Invite Reviewer | Submit Review | Make Recommendation |");
    println!("__________________________________________________________________________________________________________________________");

}

// User Actions
pub fn submit_manuscript(acm: ACM, role: &str) {
    let role_index = acm.roles.get(&String::from(role));
    
    if acm.acm[1][*role_index.unwrap()] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn invite_editors(acm: ACM, role: &str) {
        let role_index = acm.roles.get(&String::from(role));
    
    if acm.acm[1][*role_index.unwrap()] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn invite_reviewers(acm: ACM, role: &str) {
        let role_index = acm.roles.get(&String::from(role));
    
    if acm.acm[2][*role_index.unwrap()] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn submit_review(acm: ACM, role: &str) {
        let role_index = acm.roles.get(&String::from(role));
    
    if acm.acm[3][*role_index.unwrap()] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn make_recommendation(acm: ACM, role: &str) {
        let role_index = acm.roles.get(&String::from(role));
    
    if acm.acm[4][*role_index.unwrap()] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}