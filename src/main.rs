use clap::{crate_description, crate_version, App, Arg};
use std::path::Path;
use futures::executor::block_on;
use executer::{command_from_string, datetime::DateTime, runner::Runner};

fn main() {
    let matches = App::new("executer")
        .version(crate_version!())
        .author("Hossein Dindar <hosseind2017@gmail.com>")
        .about(crate_description!())
        .arg(
            Arg::with_name("file-path")
                .short("fp")
                .long("file-path")
                .value_name("file-path")
                .help("Sets the path of file that should be executed")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("runner-command")
                .short("rc")
                .long("runner-command")
                .value_name("runner-command")
                .help("Sets the runner command that should run program with")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("datetime")
                .long("datetime")
                .value_name("datetime")
                .help("Sets the date/time that should run this program, please set it in this format dd-mm-yyyy hh:mm:ss")
                .takes_value(true)
        )
        .get_matches();
    
    let runner_command = Runner::new(
        command_from_string!(matches.value_of("runner-command").unwrap()),
        &Path::new(matches.value_of("file-path").unwrap()),
    );

    let date_time = DateTime::new(
        matches.value_of("datetime").unwrap()
    );

    block_on(runner_command.execute_after_timeout(date_time.get_time_duration_to_run()));
}
