/* 
acm.rs
Cameron Fisher
Section 1
Last Modified: 11/27/22

The access control matrix module
*/

// ACM Structure
pub struct ACM{
    // Roles Array in order:
    // author, editor, associate_editor, reviewer
    roles: [[bool;5] ; 4]
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
    let acm_matrix = [author_actions, editor_actions, associate_editor_actions, reviewer_actions];

    // Create ACM struct
    let acm = ACM {roles: acm_matrix};

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
pub fn submit_manuscript() {
    println!("test");
}
pub fn invite_editors() {
    println!("test");
}
pub fn invite_reviewers() {
    println!("test");
}
pub fn submit_review() {
    println!("test");
}
pub fn make_recommendation() {
    println!("test");
}