use meow16::vm;
use strum::EnumString;
use strum::Display;

const BINARY: &str = "./meow16";
const USAGE: &str = "((<action> [file]) | <action>) [...options]";

#[derive(Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum Action {
    #[strum(serialize = "execute")]
    Execute,
    #[strum(serialize = "debug")]
    DebugStep,
    #[strum(serialize = "version")]
    WriteVersion,
    #[strum(serialize = "help")]
    WriteHelp,
}

#[derive(Default)]
struct Options {
    use_stdin: bool,
    dump_file: Option<String>,
    dump_buffer_size: usize,
    rollback_buffer_size: usize,
}

macro_rules! abort {
    ($fmt:literal $(, $($arg:tt)* )?) => {{
        eprintln!($fmt $( , $($arg)* )?);
        std::process::exit(1);
    }}
}

macro_rules! assoc {
    ($opt:expr, use $arg:expr) => {
        match ($arg) {
            Some(val) => val,
            None => abort!("expected associated value for option {}", ($opt)),
        }
    };

    ($opt:expr, $args:expr) => {
        assoc!($opt, use ($args).next())
    };
}

macro_rules! int {
    ($opt:expr, $s:expr) => {{
        let s = $s;
        match s.parse::<usize>() {
            Ok(i) => i,
            Err(_) => abort!("cannot parse int for option {} from '{}'", ($opt), s),
        }
    }}
}

macro_rules! unimplemented {
    ($msg:expr) => {
        abort!("action '{}' is not implemented", ($msg));
    }
}

fn action_execute(args: Vec<String>, options: Options) -> Result<(), vm::Error> {
    _ = args;
    _ = options;
    unimplemented!(Action::Execute);
}

fn action_debug_step(args: Vec<String>, options: Options) -> Result<(), vm::Error> {
    _ = args;
    _ = options;
    unimplemented!(Action::DebugStep);
}

fn action_write_version(args: Vec<String>, options: Options) -> Result<(), vm::Error> {
    _ = args;
    _ = options;
    unimplemented!(Action::WriteVersion);
}

fn action_write_help(args: Vec<String>, options: Options) -> Result<(), vm::Error> {
    _ = args;
    _ = options;
    unimplemented!(Action::WriteHelp);
}

fn write_usage() {
    println!("Usage: {} {}", BINARY, USAGE);
    println!("try running `{} help` for actions and options", BINARY)
}

fn main() {
    let mut options = Options::default();
    let mut args = std::env::args().skip(1).peekable();
    let mut nargs = Vec::<String>::with_capacity(16);

    if args.peek().is_none() {
        return write_usage();
    }

    let action: Action = {
        let action_string = args.next().unwrap();
        match action_string.parse() {
            Ok(action) => action,
            Err(_) => abort!("unknown action '{}'{}", action_string, if action_string.starts_with("-") {
                "; if this argument is a flag, it should be placed after the action name"
            } else { "" }),
        }
    };

    while let Some(arg) = args.next() {
        match arg {
            opt if opt == "-d"
                || opt == "--dump"
            => {
                options.dump_file = Some(assoc!(opt, args));
            },
            opt if opt == "-db"
                || opt == "--dump-buffer"
            => {
                options.dump_buffer_size = int!(opt, assoc!(opt, args));
            },
            opt if opt == "-rb"
                || opt == "--rollback-buffer"
            => {
                options.rollback_buffer_size = int!(opt, assoc!(opt, args));
            },
            opt if opt == "-" => {
                options.use_stdin = true;
            },
            s if s.starts_with("-") => abort!("unknown option {}", s),
            s => nargs.push(s),
        };
    }

    let result = match action {
        Action::Execute => action_execute(nargs, options),
        Action::DebugStep => action_debug_step(nargs, options),
        Action::WriteVersion => action_write_version(nargs, options),
        Action::WriteHelp => action_write_help(nargs, options),
    };

    let error = match result {
        Ok(_) => return,
        Err(e) => e,
    };

    abort!("{}", error.message());
}
