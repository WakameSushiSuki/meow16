use meow16::vm;
use strum::EnumString;

const USAGE: &str = "<action> [file] [...options]";

#[derive(Clone, Copy, PartialEq, Eq, EnumString)]
pub enum Action {
    Execute,
    Step,
    DebugExecute,
    DebugStep,
    WriteVersion,
    WriteHelp,
}

#[derive(Default)]
struct Options {
    dump: usize,
}

fn action_execute() -> Result<(), vm::Error> {
    Err(vm::Error::ErrorMessage("action 'execute' is not implemented".to_string()))
}

fn action_step() -> Result<(), vm::Error> {
    Err(vm::Error::ErrorMessage("action 'step' is not implemented".to_string()))
}

fn action_debug_execute() -> Result<(), vm::Error> {
    Err(vm::Error::ErrorMessage("action 'debug-execute' is not implemented".to_string()))
}

fn action_debug_step() -> Result<(), vm::Error> {
    Err(vm::Error::ErrorMessage("action 'debug-step' is not implemented".to_string()))
}

fn action_write_version() -> Result<(), vm::Error> {
    Err(vm::Error::ErrorMessage("action 'version' is not implemented".to_string()))
}

fn action_write_help() -> Result<(), vm::Error> {
    Err(vm::Error::ErrorMessage("action 'help' is not implemented".to_string()))
}

fn write_usage(program_name: &str) {
    println!("Usage: {} {}", program_name, USAGE);
    println!("Run `{} help` for help", program_name);
}

macro_rules! abort {
    ($fmt:literal $(, $($arg:tt)* )?) => {{
        eprintln!($fmt $( , $($arg)* )?);
        std::process::exit(1);
    }}
}

fn main() {
    let mut args = std::env::args().peekable();
    let program_name = args.next().unwrap();
    let options = Options::default();

    if args.peek().is_none() {
        return write_usage(&program_name);
    }

    let action: Action = {
        let action_string = args.next().unwrap();
        match action_string.parse() {
            Ok(action) => action,
            Err(_) => abort!("Unknown action '{}'", action_string),
        }
    };

    let result = match action {
        Action::Execute => action_execute(),
        Action::Step => action_step(),
        Action::DebugExecute => action_debug_execute(),
        Action::DebugStep => action_debug_step(),
        Action::WriteVersion => action_write_version(),
        Action::WriteHelp => action_write_help(),
    };

    let error = match result {
        Ok(_) => return,
        Err(e) => e,
    };

    abort!("{}", error.message());
}
