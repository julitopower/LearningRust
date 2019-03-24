use std::process;
use std::io::{self, Write};

// Let's define a struct to invoke GCC
pub struct Gcc {
    files: Vec<String>,
    out: String,
}

impl Gcc {
    fn new() -> Gcc {
        Gcc {
            files: vec![],
            out: "a.out".to_string(),
        }
    }

    fn add_file(&mut self, file: &str) -> &mut Gcc {
        self.files.push(file.to_string());
        self
    }

    fn out(&mut self, file: &str) -> &mut Gcc {
        (self.out) = file.to_string();
        self
    }

    fn call(&mut self) -> &mut Gcc {
        // Prepare the command
        let mut cmd = process::Command::new("gcc");
        for f in &self.files {
            cmd.arg(f);
        }
        cmd.arg("-o").arg(&self.out);

        // Call the command
        let output = cmd.output().expect("Something when wrong");
        if !output.status.success() {
            io::stdout().write_all(&output.stderr).expect(
                "Couldn't write",
            );
        } else {
            io::stdout().write_all(&output.stdout).expect(
                "Couldn't write",
            );
        }
        self
    }
}

fn main() {
    // "Manually" calling a process
    let mut cmd = process::Command::new("ls");
    cmd.arg("-lah");
    let output = cmd.output().expect("Something when wrong");
    io::stdout().write_all(&output.stdout).expect(
        "Couldn't write",
    );

    // Using Gcc to abstract calling out to Gcc
    let gcc = Gcc::new().add_file("main.c").out("app").call();
}
