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
    // Roles Array (Row, 1st num) in order:
    // author, editor, associate_editor, reviewer
    // Actions/Permissions (Column, 2nd num) array in order:
    // submit, invite_associates, invite_reviewers, submit_review, make_recommendation
    // Usage: acm[role_index][permission_index]
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
    roles.insert(String::from("author"), 0);
    roles.insert(String::from("editor"), 1);
    roles.insert(String::from("associate editor"), 2);
    roles.insert(String::from("reviewer"), 3);

    // Create ACM struct
    let acm = ACM {acm: ac_matrix, roles: roles};

    acm
}

// For admin printing ACM (also for debugging)
pub fn print_acm(acm: ACM) {
    println!("__________________________________________________________________________________________________________________________");
    println!("|       Role       | Submit Manuscript | Invite Associate Editor | Invite Reviewer | Submit Review | Make Recommendation |");
    println!("__________________________________________________________________________________________________________________________");
    println!("|      Author      |       {}       |          {}          |      {}      |     {}     |        {}        |",
            acm_str(acm.acm[0][0]), acm_str(acm.acm[0][1]), acm_str(acm.acm[0][2]), acm_str(acm.acm[0][3]), acm_str(acm.acm[0][4]));
    println!("__________________________________________________________________________________________________________________________");
    println!("|      Editor      |       {}       |          {}          |      {}      |     {}     |        {}        |",
            acm_str(acm.acm[1][0]), acm_str(acm.acm[1][1]), acm_str(acm.acm[1][2]), acm_str(acm.acm[1][3]), acm_str(acm.acm[1][4]));
    println!("__________________________________________________________________________________________________________________________");
    println!("| Associate Editor |       {}       |          {}          |      {}      |     {}     |        {}        |",
            acm_str(acm.acm[2][0]), acm_str(acm.acm[2][1]), acm_str(acm.acm[2][2]), acm_str(acm.acm[2][3]), acm_str(acm.acm[2][4]));
    println!("__________________________________________________________________________________________________________________________");
    println!("|     Reviewer     |       {}       |          {}          |      {}      |     {}     |        {}        |",
            acm_str(acm.acm[3][3]), acm_str(acm.acm[3][1]), acm_str(acm.acm[3][2]), acm_str(acm.acm[3][3]), acm_str(acm.acm[3][4]));
    println!("__________________________________________________________________________________________________________________________");

}

// For formatting the table printing to look neat
fn acm_str(perm: bool) -> &'static str {
    if perm {
        return " true"
    }
    return "false"
}

// User Actions
pub fn submit_manuscript(acm: ACM, role: &str) {

    // Check permissions in ACM
    let role_index = acm.roles.get(&String::from(role));
    if acm.acm[0][*role_index.unwrap()] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn invite_editors(acm: ACM, role: &str) {

    // Check permissions in ACM
    let role_index = acm.roles.get(&String::from(role));
    if acm.acm[*role_index.unwrap()][1] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn invite_reviewers(acm: ACM, role: &str) {

    // Check permissions in ACM
    let role_index = acm.roles.get(&String::from(role));
    if acm.acm[*role_index.unwrap()][2] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn submit_review(acm: ACM, role: &str) {

    // Check permissions in ACM
    let role_index = acm.roles.get(&String::from(role));
    if acm.acm[*role_index.unwrap()][3] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}


pub fn make_recommendation(acm: ACM, role: &str) {

    // Check permissions in ACM
    let role_index = acm.roles.get(&String::from(role));
    if acm.acm[*role_index.unwrap()][4] == true {
        println!("Allowed")
    } else {
        println!("Not Allowed")
    }
}