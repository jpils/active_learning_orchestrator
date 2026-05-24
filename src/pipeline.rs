use std::{path::Path, process::{Command, Output}};
use serde_macros::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Step { 
    name: String,
    command: String,
    args: Vec<String>,
    working_dir: String
}

#[derive(Debug)]
pub struct Pipeline {
    pub(crate) commands: Vec<Command>
}

impl Pipeline {
    fn run(mut pipeline: Vec<Command>) -> Vec<Output>{
        pipeline
            .iter_mut()
            .map(|step| step.output().unwrap())
            .collect::<Vec<_>>()
    }
}

