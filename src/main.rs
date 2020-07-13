use clap::{value_t, App, Arg};
use psutil::process::Process;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("pid")
                .short("p")
                .long("pid")
                .help("Process ID")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .help("Sampling interval in milliseconds")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("log")
                .help("Log level")
                .takes_value(true)
                .possible_values(&["trace", "debug", "info", "warn", "error"])
                .case_insensitive(true)
                .default_value("info"),
        )
        .get_matches();

    let pid = value_t!(matches.value_of("pid"), u32).unwrap();
    let interval = value_t!(matches.value_of("interval"), u64).unwrap();
    let verbosity = matches.value_of("verbosity").unwrap();

    let mut process = Process::new(pid).unwrap();

    loop {
        println!(
            "RAM: {:>4}%\tCPU: {:>4}%",
            process.memory_percent().unwrap(),
            process.cpu_percent().unwrap()
        );
        std::thread::sleep(std::time::Duration::from_millis(interval));
    }
}
