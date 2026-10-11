use std::os::unix::fs::PermissionsExt;
use std::{env, io, fs};
use std::process::{exit, id, Command};
fn main() {
    // intro
    let borderchar = "------------------------------------------------------";
    println!("SLURM Job Submission");
    println!("{borderchar}");

    // get git url for repo to be cloned
    println!("Please enter the url of the git repository to be used (ex: https://github.com/my_account/repo_name.git");
    let mut git_url = String::new();
    io::stdin().read_line(&mut git_url).expect("Could not read from stdin!");

    // rclone configuration:

    // get home dir
    let home = env::var("HOME").unwrap_or_else(|e| {
        eprintln!("Error: Could not capture home environment variable! Full Error: {e}");
        exit(1);
    });

    // get pid
    let id = id();

    // build rclone config dir
    let config_dir = format!("{home}/git_slurm_ui/{id}");

    // create config dir (uses create_dir_all to create all nested dirs in provided path)
    fs::create_dir_all(&config_dir).unwrap_or_else(|e| {
        eprintln!("Error: Could not create configuration directory! Full Error: {e}");
    });

    // build rclone config file path:
    let config_file = format!("{config_dir}/rclone.conf");

    // issue rclone command to open browser for user to authenticate:
    let rcmd = Command::new("rclone").args(["config", "create", "gdrive", "drive", "config_is_local", "true", "--config", &config_file]).status().unwrap_or_else(|e| {
        eprintln!("Could not launch rclone! Is it installed? Full Error: {e}");
        exit_rclone_rm(&config_dir);
    });

    // if it fails, make sure configuration dir is removed before exit:
    if !rcmd.success() {
        eprintln!("Error with rclone! git_slurm_ui terminatng! See above error messages!");
        exit_rclone_rm(&config_dir);
    }


    // set permissions on rclone config path so others can not read it:
    fs::set_permissions(&config_file, fs::Permissions::from_mode(0o700)).unwrap_or_else(|e| {
        eprintln!("Error: Could not set config file permissions! Full error: {e}");
        exit_rclone_rm(&config_dir);
    });

    // Ask if project has been approved
    println!("Has this job been approved by faculty to run on the computer?");
    println!("\'Y\' for Yes and \'N\' for No");
    let mut approved = String::new();
    io::stdin().read_line(&mut approved).expect("IO Error: Failed to read input from terminal!");
    let approved = approved.trim();

    // check if it is empty
    if approved.is_empty() {
        eprintln!("Error: No approval response provided! Please obtain approval from faculty if not already approved and try again!");
        exit_rclone_rm(&config_dir);
    }

    // if not approved, remove configuration file and exit
    if approved != "Y" && approved != "y" {
        eprintln!("Error! Job needs to be manually reviewed and approved before being submitted! Get approval from facutly and submit again!");
        exit_rclone_rm(&config_dir);
    }

    // Ask for the job name
    println!("What should the job name be?");
    let mut job_name = String::new();
    io::stdin().read_line(&mut job_name).expect("IO Error: Failed to read input from terminal!");
    let job_name = job_name.trim();

    if job_name.is_empty() {
        eprintln!("Error: No job name provided!");
        exit_rclone_rm(&config_dir);
    }

    // submit slurm job
    let slurm_workdir = format!("{home}/git_slurm_ui/slurm_%j");
    
    let sbat = Command::new("sbatch").args(["--job-name", &job_name, "--output", &slurm_workdir])
    .arg("slurm_sbatch.sh")
    .arg(&git_url)
    .arg(&config_file)
    .status().unwrap_or_else(|e| {
        eprintln!("Error: Could not issue sbatch command! Is it installed? Full error: {e}");
        exit_rclone_rm(&config_dir);
    });

    if !sbat.success() {
        eprintln!("Error with sbatch command! git_slurm_ui terminatng! See above error messages!");
        exit_rclone_rm(&config_dir);
    }

}

// function to remove config file or warn user, so this code does not need to be repeated with every possible error
fn exit_rclone_rm(path: &str) -> ! {
    fs::remove_dir_all(path).unwrap_or_else(|e| {
        let emsg = format!("Error: Failed to remove rclone config, student account tokens are still present! Please remove the directory: {path} Full error: {e}");
        eprintln!("{emsg}");
        exit(1);
    });
    exit(1);
}