use std::io::stdin;
use std::process::exit;
use dialoguer::Password;
use keyring::Entry;
use secrecy::{SecretString, ExposeSecret};

fn main() {
    println!("SLURM JOB SUBMISSION");

    // get github username
    let mut gh_username = String::new();
    println!("Please enter a valid github username: ");
    stdin().read_line(&mut gh_username).expect("IO Error: Failed to read input from terminal!");
    // must be trimmed to remove new line:
    let gh_username = gh_username.trim();

    if gh_username.is_empty() {
        eprintln!("Error: No username provided!");
        exit(1);
    }

    // get project repo name:
    let mut repo_name = String::new();
    println!("Please enter the GitHub repository name of the project you wish to run: ");
    stdin().read_line(&mut repo_name).expect("IO Error: Failed to read input from terminal!");
    let repo_name = repo_name.trim();

    if repo_name.is_empty() {
        eprintln!("Error: No repository name provided!");
        exit(1);
    }

    // Ask if project has been approved
    println!("Has this job been approved by faculty to run on the computer?");
    println!("\'Y\' for Yes and \'N\' for No");
    let mut approved = String::new();
    stdin().read_line(&mut approved).expect("IO Error: Failed to read input from terminal!");
    let approved = approved.trim();

    if approved.is_empty() {
        eprintln!("Error: No approval response provided! Please obtain approval from faculty if not already approved and try again!");
        exit(1);
    }

    if approved != "Y" && approved != "y" {
        eprintln!("Error! Job needs to be manually reviewed and approved before being submitted! Get approval from facutly and submit again!");
        exit(1);
    }

    // Construct github repo url:
    let repo_url = format!("https://github.com/{gh_username}/{repo_name}.git");

    // SECURELY get project PAT
    let inp = Password::new().with_prompt("Please enter the personal access token to your github account so results can be uploaded").interact().unwrap_or_else(|e|      
    {
        eprintln!("Error grabbing access token: {e}");
        exit(1);
    });
    // .into() converts it into a boxed string (b/c that is the kind the func. expects)
    let pat = SecretString::new(inp.into());

    // Put PAT into OS keyring for secure storage!
    let key_entry = Entry::new("DGX_GH_Slurm_Frontend", &gh_username).unwrap_or_else(|e| {
        eprintln!("Not a valid keyring entry, error: {e}");
        exit(1);
    });

    key_entry.set_password(pat.expose_secret()).unwrap_or_else(|e| 
    {
        eprintln!("Could not set keyring entry!, error: {e}");
        exit(1);
    });

    // Ask for the job name
    println!("What should the job name be?");
    let mut job_name = String::new();
    stdin().read_line(&mut job_name).expect("IO Error: Failed to read input from terminal!");
    let job_name = job_name.trim();

    if job_name.is_empty() {
        eprintln!("Error: No job name provided!");
        exit(1);
    }

    println!("Your job has been queued!");
}
