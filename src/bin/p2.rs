// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
struct Records {
    records: HashMap<i64, Record>,
}

impl Records {
    fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    fn add(&mut self, record: Record) {
        self.records.insert(record.id, record);
    }

    fn into_vec(mut self) -> Vec<Record> {
        let mut vec: Vec<_> = self.records.drain().map(|kv| kv.1).collect();
        vec.sort_by_key(|r| r.id);
        vec
    }

    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.records.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1,
        }
    }

    fn search(&self, name: &str) -> Vec<&Record> {
        let mut results = Vec::new();
        for record in self.records.values() {
            if record.name.to_lowercase().contains(&name.to_lowercase()) {
                results.push(record);
            }
        }
        results
    }

    fn remove(&mut self, id: i64) -> Option<Record> {
        self.records.remove(&id)
    }

    fn edit(&mut self, id: i64, name: &str, email: Option<String>) {
        self.records.insert(
            id,
            Record {
                id,
                name: name.to_string(),
                email: email,
            },
        );
    }
}

fn save_records(file_name: PathBuf, records: Records) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;

    file.write(b"id,name,email\n")?;

    for record in records.into_vec().into_iter() {
        let email = match record.email {
            Some(email) => email,
            None => String::new(),
        };
        let line = format!("{},{},{}\n", record.id, record.name, email);
        file.write(line.as_bytes())?;
    }
    file.flush()?;
    Ok(())
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("id must be a number but was `{0}`")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("empty record")]
    EmptyRecord,
    #[error("missing field: {0}")]
    MissingField(String),
}

fn parse_record(record: &str) -> Result<Record, ParseError> {
    let fields: Vec<&str> = record.split(',').collect();
    let id = match fields.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };

    let name = match fields.get(1).filter(|name| **name != "") {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };

    let email = fields
        .get(2)
        .map(|email| email.to_string())
        .filter(|email| email != "");
    

    Ok(Record { id, name, email })
}

fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    for (num, record) in records.split('\n').enumerate() {
        if record != "" {
            match parse_record(record) {
                Ok(rec) => {
                    recs.add(rec);
                }
                Err(e) => {
                    if verbose {
                        println!("Error parsing record {:?}: {:?}", num, e);
                    }
                }
            }
        }
    }
    recs
}


fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(parse_records(contents, verbose))
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, parse(from_os_str), default_value = "p2_data.csv")]
    data_file: PathBuf,

    #[structopt(subcommand)]
    cmd: Command,

    #[structopt(short, help = "Print verbose output")]
    verbose: bool,
}

#[derive(Debug, StructOpt)]
enum Command {
    List {},
    Add { name: String, #[structopt(short)] email: Option<String> },
    Search { query: String },
    Remove { id: i64 },
    Edit { id: i64, name: String, #[structopt(short)] email: Option<String> },
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        Command::Add { name, email } => {
            let mut records = load_records(opt.data_file.clone(), opt.verbose)?;
            let id = records.next_id();
            records.add(Record { id, name, email });
            save_records(opt.data_file, records)?;
        },
        Command::List { .. } => {
            let records = load_records(opt.data_file, opt.verbose)?;
            for record in records.into_vec() {
                println!("{:?}", record);
            }
        },
        Command::Search { query } => {
            let records = load_records(opt.data_file, opt.verbose)?;
            let results = records.search(&query);
            if results.is_empty() {
                println!("No results found");
            } else {
                for record in results {
                    println!("{:?}", record);
                }
            }
        },
        Command::Remove { id } => {
            let mut records = load_records(opt.data_file.clone(), opt.verbose)?;
            match records.remove(id) {
                Some(record) => {
                    println!("Removed {:?}", record);
                    save_records(opt.data_file, records)?;
                },
                None => {
                    println!("No record with id {}", id);
                },
            }
        },
        Command::Edit { id, name, email } => {
            let mut records = load_records(opt.data_file.clone(), opt.verbose)?;
            records.edit(id, &name, email);
            save_records(opt.data_file, records)?;
        },
    }
    Ok(())
}

fn main()
{
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
