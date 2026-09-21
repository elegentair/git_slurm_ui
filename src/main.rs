use std::io::stdin;
use dialoguer::Password;
use keyring::{Entry, Result as KeyResult};
use secrecy::{SecretString, ExposeSecret};

fn main() {
    println!("SLURM JOB SUBMISSION");

    // get github username
    let mut gh_username = String::new();
    println!("Please enter a valid github username: ");
    stdin().read_line(&mut gh_username).expect("Please quit and enter a valid url!");
    // must be trimmed to remove new line:
    let gh_username = gh_username.trim();

    // get project repo name:
    let mut repo_name = String::new();
    println!("Please enter the GitHub repository name of the project you wish to run: ");
    stdin().read_line(&mut repo_name).expect("Please enter a valid name!");
    let repo_name = repo_name.trim();

    // SECURELY get project PAT
    let inp = Password::new().with_prompt("Please enter the personal access token to your github account so results can be uploaded").interact().unwrap_or_else(|e| {panic!("Error {e}!")});
    // .into() converts it into a boxed string (b/c that is the kind the func. expects)
    let pat = SecretString::new(inp.into());

    // Put PAT into OS keyring for secure storage!
    let key_entry = Entry::new("DGX_GH_Slurm_Frontend", &gh_username).unwrap_or_else(|e| {panic!("Not a valid keyring entry, error: {e}")});

    key_entry.set_password(pat.expose_secret()).unwrap_or_else(|e| {panic!("Could not set keyring entry!, error: {e}")});


    // Ask if project has been approved
    println!("Has this job been approved by faculty to run on the computer?");
    println!("\'Y\' for Yes and \'N\' for No");
    let mut approved = String::new();
    stdin().read_line(&mut approved).expect("Please enter \'Y\' or \'N\' and try again!");
    let approved = approved.trim();

    if approved != "Y" && approved != "y" {
        panic!("Panicing! Job needs to be manually reviewed and approved before being submitted!");
    }

    // Ask for the job name
    println!("What should the job name be?");
    let mut job_name = String::new();
    stdin().read_line(&mut job_name).expect("Please enter a valid name!");
    let job_name = job_name.trim();

    println!("Your job has been queued!");
}
