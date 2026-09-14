use std::io::stdin;
fn main() {
    println!("SLURM JOB SUBMISSION");

    let mut git_url = String::new();
    println!("Please enter a valid git repository link to grab results from (i.e your github repo):");
    stdin().read_line(&mut git_url).expect("Please quit and enter a valid url!");

    println!("Please enter the personal access token to your github account so results can be uploaded.");
    let mut pat = String::new();
    stdin().read_line(&mut pat).expect("Please quit and enter a valid personal access token!");

    println!("Has this job been approved by faculty to run on the computer?");
    println!("\'Y\' for Yes and \'N\' for No");
    let mut approved = String::new();
    stdin().read_line(&mut approved).expect("Please enter \'Y\' or \'N\' and try again!");

    if approved != "Y" {
        panic!("Panicing! Job needs to be manually reviewed and approved before being submitted!");
    }

    println!("What should the job name be?");
    let mut job_name = String::new();
    stdin().read_line(&mut job_name).expect("Please enter a valid name!");

    println!("Your job has been queued!");
}
