use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::channel,
    thread,
    time::Duration,
};

const MAX: u16 = 65535;

fn scan(addr: IpAddr, threads: u16) {
    let (tx, rx) = channel();

    for i in 0..threads {
        let tx = tx.clone();

        thread::spawn(move || {
            let mut port: u16 = i + 1;
            loop {
                match TcpStream::connect(format!("{}:{}", addr, port)) {
                    Ok(_) => {
                        print!(".");
                        io::stdout().flush().unwrap();
                        tx.send(port).unwrap();
                        thread::sleep(Duration::from_secs(1));
                    }
                    Err(_) => {}
                }

                if (MAX - port) <= threads {
                    break;
                }

                port += threads;
            }
        });
    }

    let mut out = vec![];

    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");

    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!(
            "Error: You need to specify a command to run. Run -h or --help for more information."
        );
        std::process::exit(1);
    }

    let command = args[1].clone();

    if command == "-h" || command == "--help" {
        println!("Usage: {} [command] [arguments]", args[0]);
        println!(" -h, --help\tDisplay this help message");
        println!(" -s, --scan\tScan an IP address. Usage: -s [IPv4 Address]");
    } else if command == "-s" || command == "--scan" {
        if args.len() == 2 {
            println!("Usage: {} [command] [arguments]", args[0]);
            println!(" -h, --help\tDisplay this help message");
            println!(" -s, --scan\tScan an IP address. Usage: -s [IPv4 Address]");
            process::exit(1);
        }

        let threads = match args[2].parse::<u16>() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Failed to parse thread number");
                process::exit(1);
            }
        };

        let ip = match IpAddr::from_str(&args[3]) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Invalid IP");
                process::exit(1);
            }
        };

        scan(ip, threads);
    } else {
        println!("Usage: {} [command] [arguments]", args[0]);
        println!(" -h, --help\tDisplay this help message");
        println!(" -s, --scan\tScan an IP address. Usage: -s [IPv4 Address]");
        process::exit(1);
    }

    std::process::exit(0);
}
