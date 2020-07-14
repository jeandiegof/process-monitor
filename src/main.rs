use chrono::Local;
use clap::{value_t, App, Arg};
use csv::WriterBuilder;
use psutil::process::Process;
use std::fs::OpenOptions;
use std::path::Path;

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
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pid = value_t!(matches.value_of("pid"), u32).unwrap();
    let interval_ms = value_t!(matches.value_of("interval"), u64).unwrap();
    let output = matches.value_of("output").unwrap();

    let mut process = Process::new(pid).unwrap();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(Path::new(output))
        .unwrap();

    let mut writer = WriterBuilder::new()
        .has_headers(file.metadata().unwrap().len() == 0)
        .from_writer(file);

    writer
        .write_record(&["date", "time", "ram_percent", "cpu_percent"])
        .unwrap();

    loop {
        let ram = process.memory_percent().unwrap();
        let cpu = process.cpu_percent().unwrap();
        let date = Local::now().date().format("%Y-%m-%d").to_string();
        let time = Local::now().time().format("%H:%M:%S%.3f").to_string();

        println!("RAM: {:>4}%\t\tCPU: {:>4}%", ram, cpu);
        writer
            .write_record(&[date, time, ram.to_string(), cpu.to_string()])
            .unwrap();
        writer.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(interval_ms));
    }
}
