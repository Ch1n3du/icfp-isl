use icfp_isl::cli;

fn main() {
    let mut args = std::env::args();
    args.next();

    match args.next() {
        Some(command) => match command.as_str() {
            "repl" => {
                let mut verbose = false;
                if let Some(verbose_str) = args.next() {
                    verbose = verbose_str == "-v" || verbose_str == "--v";
                }

                cli::run_repl(verbose)
            }
            "run" => {
                let file_name: String;

                if let Some(file_name_) = args.next() {
                    file_name = file_name_;
                } else {
                    panic!("CLI Error: No filename supplied for 'run'.")
                }

                let mut verbose = false;
                if let Some(verbose_str) = args.next() {
                    verbose = verbose_str == "-v" || verbose_str == "--v";
                }

                cli::run_file(file_name, verbose);
            }
            _ => {
                panic!("CLI Error: Unknown command.")
            }
        },
        _ => return,
    }
}
