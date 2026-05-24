use tokio::process::Command;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting the orchestrator...");

    let handle = tokio::spawn(async {
        Command::new("sleep").arg("5").status().await
    });

    println!("I am not waiting for the sleep command!");
    println!("I can do other things here, like monitor other jobs.");

    let status = handle.await??;

    println!("Done!");
    println!("{:#?}", status);
    Ok(())
}
