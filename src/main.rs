/*
Commands
ip_sniffer.exe -h   ...help
ip_sniffer.exe -j 100 192.168.1.1  ...# of threads, ip
ip_sniffer.exe 192.168.1.1  ... ip
 */


use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX_PORT: u16 = 65535;

struct Arguments
{
    flag: String,
    ip_address: IpAddr,
    threads: u16,
}

impl Arguments
{
    fn new(args: &[String]) -> Result<Arguments, &'static str>
    {
        if args.len() < 2
        {
            return Err("Too few arguments!");
        } else if args.len() > 4
        {
            return Err("Too many arguments!");
        }
        let f = args[1].clone();
        if let Ok(ip_address) = IpAddr::from_str(&f)
        {
            return Ok(Arguments { flag: String::from(""), ip_address, threads: 4 });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2
            {
                println!("Usage: -j to select the amount of threads you want\
                \r\n    -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help")
            {
                return Err("Too many arguments!");
            } else if flag.contains("-j")
            {
                let ip_address = match IpAddr::from_str(&args[3])
                {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>()
                {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };
                return Ok(Arguments { threads, flag, ip_address });
            } else {
                return Err("invalid syntax...");
            }
        }
    }
}

fn scan(transmitter_x: Sender<u16>, start_port: u16, address: IpAddr, num_threads: u16)
{
    let mut port:u16 = start_port + 1;
    loop
    {
        match TcpStream::connect((address, port))
        {
            Ok(_) =>
                {
                    print!(".");    //returned for every open port
                    io::stdout().flush().unwrap();
                    transmitter_x.send(port).unwrap();
                }
            Err(_) => {}
        }
        if (MAX_PORT - port) <= num_threads
        {
            break;
        }
        port += num_threads;
    }
}

fn main()
{
    //place all arg passed in a vector of strings
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err|
            {
                if err.contains("help")
                {
                    process::exit(0);
                } else {
                    eprintln!("{} problem parsing arguments: {}", program, err);
                    process::exit(0);
                }
            }
    );

    let number_of_threads = arguments.threads;
    let address = arguments.ip_address;
    let (transmitter_x, receiver_x) = channel();
    for i in 0..number_of_threads
    {
        let transmitter_x = transmitter_x.clone();

        thread::spawn(move ||
            {
                scan(transmitter_x, i, address, number_of_threads);
            });
    }

    let mut out = vec![];
    drop(transmitter_x);
    for p in receiver_x
    {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out{
        println!("{} is open", v);
    }
}
