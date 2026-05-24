fn create_command(step: Step) -> Command {
    let mut command = Command::new(step.command);
    for arg in step.args {
        command.arg(arg);
    }

    command
}
