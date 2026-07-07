use meow16::vm;
use strum::EnumString;

const USAGE: &str = "((<action> [file]) | <action>) [...options]";

#[derive(Clone, Copy, PartialEq, Eq, EnumString)]
pub enum Action {
    #[strum(serialize = "execute")]
    Execute,
    #[strum(serialize = "step")]
    Step,
    #[strum(serialize = "debug-execute")]
    DebugExecute,
    #[strum(serialize = "debug-step")]
    DebugStep,
    #[strum(serialize = "version")]
    WriteVersion,
    #[strum(serialize = "help")]
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

fn opt_val(key: &str, val: Option<String>) -> String {
    match val {
        Some(val) => val,
        None => abort!("expected key for option {}", key),
    }
}

fn opt_int(opt: &str, s: String) -> usize {
    match s.parse::<usize>() {
        Ok(i) => i,
        Err(_) => abort!("cannot parse int for option {} from '{}'", opt, s),
    }
}

fn main() {
    let mut options = Options::default();
    let mut args = std::env::args().peekable();
    let program_name = args.next().unwrap();

    if args.peek().is_none() {
        return write_usage(&program_name);
    }

    let action: Action = {
        let action_string = args.next().unwrap();
        match action_string.parse() {
            Ok(action) => action,
            Err(_) => abort!("unknown action '{}'", action_string),
        }
    };

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            opt @ ("-d" | "--dump") => {
                options.dump = opt_int(opt, opt_val(opt, args.next()));
            },
            "-" => todo!(), // sets file to stdin (once i handle getting the file)
            s if s.starts_with("-") => abort!("unknown option {}", s),
            _ => (),
        };
    }

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
