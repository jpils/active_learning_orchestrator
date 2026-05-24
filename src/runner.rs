use std::process::Output;

trait Runner {
    fn run(&self) -> anyhow::Result<Output>;
} 

struct LocalRunner {
    
}

impl Runner for LocalRunner {
    fn run(&self) -> anyhow::Result<Output> {
        todo!()
    }
}

struct SlurmRunner {

}

impl Runner for SlurmRunner {
    fn run(&self) -> anyhow::Result<Output> {
        todo!()
    }
}
