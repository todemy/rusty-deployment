use clap;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use std::thread;

#[derive(Debug, Deserialize, Clone)]
struct Step {
    relative_dir: Option<String>,
    name: String,
    command: String,
    args: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Clone)]
struct Section {
    name: String,
    steps: Vec<Step>
}

#[derive(Debug, Deserialize)]
struct DeployConfig {
    sections: Vec<Section>
}

fn retrieve_config<P: AsRef<Path>>(path: P) -> Result<Box<DeployConfig>, Box<Error>> {

    let file = File::open(path)?;

    let config = serde_json::from_reader(file)?;

    Ok(config)
}


fn run_individual_step(step: &Step) -> Output {
    let mut command_process = Command::new(&step.command);

    match step.relative_dir {
        Some(ref dir) => {
            command_process.current_dir(dir);
        },
        _ => {}
    };

    match step.args {
        Some(ref args) => {
            for arg in args.iter() {
                command_process.arg(arg);
            }
        },
        _ => {}
    };

    command_process.output().expect("failed to run command")
}


fn deploy(config: String) {
    let deploy_config = retrieve_config(config).expect("Unable to find correct configuration file");

    for section in deploy_config.sections.into_iter() {
        println!("Starting section : {}\n\n\n", section.name);
        let mut step_threads = vec![];

        for step in section.steps.into_iter() {
            step_threads.push(thread::spawn(move || {
                let output = run_individual_step(&step);
                if output.status.success() {
                    println!("Completed step {}", step.name);
                    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("Failed to run step: {} : {}", step.name, String::from_utf8_lossy(&output.stderr));
                }
            }))
        };

        for thread in step_threads {
            let _ = thread.join().unwrap();
        };

        println!("\n\nCompleted section: {}\n\n", section.name);
    }

    println!("\nDone.")
}

pub fn deploy_all(environments: clap::Values) {

    for environment in environments {
        match environment.to_string().to_uppercase().as_ref() {
            "PROD" => {
                deploy(String::from("tody-deploy-config-prod.json"))
            },
            "DEV" => {
                deploy(String::from("tody-deploy-config-dev.json"))
            },
            _ => println!("Unable to deploy environment provided: {}", environment.to_string())
        };
    }
}