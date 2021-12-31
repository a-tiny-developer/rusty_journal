use rusty_journal::{
    cli::{Action::*, CommandLineArgs},
    tasks::{self, Task},
};
use structopt::StructOpt;

fn main() {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    
    // Unpack the journal file.
    let journal_file = journal_file.expect("Failed to find journal file");

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}
