use std::{
    env,
    fs,
    thread,
};
use std::process::{
    Command,
    exit,
};
use std::fs::File;
use std::time::Duration;
use std::thread::sleep;
use std::io::prelude::*;
use std::process;
use std::path::Path;
use nix::unistd::Uid;

fn take_thread(s: String){

      let _handle = thread::spawn(move || {
          println!("Now applying paxctl to {}", &s);
          Command::new("paxctl") 
            .args(["-c", &s])
            .output()
            .expect("Failed to execute process!");
          Command::new("paxctl")
            .arg("-PEMRXS")
            .arg(&s)
            .output()
            .expect("Failed to execute process!");
      });
}

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {

    if !Path::new("/usr/sbin/paxctl").exists() {
        println!("Paxctl was not found.\nPlease install this and try again.");
        process::exit(0x0100);
    }
    
    if !Uid::effective().is_root() {
        println!("You must be root/sudo/doas. Come back when you are.");
        process::exit(0x0100);
    }

    unsafe { // Set priority on this process to 15.
        libc::setpriority(libc::PRIO_PROCESS, 0, 15);
    }

    let foo = match env::var("PATH") {
        Ok(val) => val,
        Err(_e) => "Failed to get $PATH variable.".to_string(),
    };

    let output = foo.replace(":", "\n"); // Replace : in $PATH with new lines.
    println!("About to commit 'paxctl -PEMRXS' to all ELF binaries in $PATH directories.");
    println!("Would you like to proceed? [Y/N] ");

    let mut line = String::new();
    let _input = std::io::stdin().read_line(&mut line).unwrap();

    match line.as_str().trim() {
        "Y" | "y" => { println!("This will take some time to complete. Go make some coffee.");
        sleep(Duration::from_millis(2000)) },
        "N" | "n" => { println!("As you wish. Exiting.");
        exit(0); },
        _ => { println!("Valid input was not received. Exiting.");
        exit(1); },
    } 
   
    let strings: Vec<_> = output.lines().collect();

    for a in strings {
        let mut thread_handles = vec![];
        let entries = fs::read_dir(a.to_string()).unwrap();
        for entry in entries {
            // Get files in directories taken from $PATH.
            let entry = entry.unwrap();
            if entry.file_type()?.is_symlink() {
                continue;
            }
            let b = entry.path().display().to_string();
            let mut f = File::open(&b)?;
            let mut buffer = [0; 4];
            match f.read_exact(&mut buffer) {
                Ok(()) if &buffer == b"\x7fELF" => {
                    thread_handles.push(thread::spawn(move || {
                        take_thread(b);
                    }));
                }
                Err(..) => continue,
                _ => continue,
            }
            sleep(Duration::from_millis(50)); // Added to prevent too many open files errors.
        }
        // Wait for threads and join.
        let _strings = thread_handles.into_iter().map(|h| h.join().unwrap());
    }

    println!("\nAll ELF binaries in $PATH have been updated.");
    println!("You may confirm with 'paxctl -v /path/to/binary'");
    println!("Keep in mind, RANDEXEC is not supported beyond kernel 2.6");

    Ok(())
}
