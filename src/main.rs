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
    write_end: bool,
    dump_file: Option<String>,
    dump_buffer_size: usize,
    rollback_buffer_size: usize,
}

macro_rules! abort {
    ($fmt:literal $(, $($arg:tt)* )?) => {{
        eprint!($fmt $( , $($arg)* )?);
        std::process::exit(1);
    }}
}

macro_rules! assoc {
    ($opt:expr, use $arg:expr) => {
        match ($arg) {
            Some(val) => val,
            None => abort!("expected associated value for option {}\n", ($opt)),
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
            Err(_) => abort!("cannot parse int for option {} from '{}'\n", ($opt), s),
        }
    }}
}

fn action_execute(_args: Vec<String>, _options: Options) -> Result<(), vm::Error> {
    todo!("action '{}'", Action::Execute);
}

fn action_debug_step(_args: Vec<String>, _options: Options) -> Result<(), vm::Error> {
    todo!("action '{}'", Action::DebugStep);
}

fn action_write_version(_args: Vec<String>, _options: Options) -> Result<(), vm::Error> {
    todo!("action '{}'", Action::WriteVersion);
}

fn action_write_help(_args: Vec<String>, _options: Options) -> Result<(), vm::Error> {
    todo!("action '{}'", Action::WriteHelp);
}

fn write_usage() {
    println!("Usage: {} {}", BINARY, USAGE);
    println!("try running `{} help` for actions and options", BINARY)
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let mut msg = String::with_capacity(256);
        msg.push_str("[[HOST]] an error occured");

        let s: &str = if let Some(s) = info.payload().downcast_ref::<&'static str>() {
            msg.push('\n');
            s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            msg.push('\n');
            s
        } else {
            ""
        };

        for ln in s.lines() {
            msg.push_str("    ");
            msg.push_str(ln);
            msg.push('\n');
        }

        if let Some(t) = std::thread::current().name() {
            if t != "main" {
                msg.push_str("non-main thread '");
                msg.push_str(t);
                msg.push('\'');
                msg.push('\n');
            }
        } else {
            msg.push_str("non-main thread\n");
        }

        abort!("{}", msg);
    }));

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
            Err(_) => abort!("unknown action '{}'{}\n", action_string, if action_string.starts_with("-") {
                "; if this argument is a flag, it should be placed after the action name"
            } else { "" }),
        }
    };

    while let Some(arg) = args.next() {
        match arg {
            opt if opt == "-" => {
                options.use_stdin = true;
            },
            opt if opt == "-e"
                || opt == "--write-end"
            => {
                options.write_end = true;
            }
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
            s if s.starts_with("-") => abort!("unknown option {}\n", s),
            s => nargs.push(s),
        };
    }

    let wend = options.write_end;

    let result = match action {
        Action::Execute => action_execute(nargs, options),
        Action::DebugStep => action_debug_step(nargs, options),
        Action::WriteVersion => action_write_version(nargs, options),
        Action::WriteHelp => action_write_help(nargs, options),
    };

    let error = match result {
        Ok(_) => return if wend {
            println!("[[VM]] end of program");
            ()
        },
        Err(e) => e,
    };

    abort!("[[VM]] {}\n", error.message());
}
