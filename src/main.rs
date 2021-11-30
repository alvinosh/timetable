use chrono::prelude::*;
use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // TODO: table set [day] [hour] [value]

    const DAYS: usize = 5;
    const HOURS: usize = 7;

    let mut timetable: Vec<String> = vec![];

    if let Some(proj_dirs) = ProjectDirs::from("", "Vinc", "table") {
        let path = proj_dirs.data_dir();
        fs::create_dir_all(path).expect("Could not create folder");
        let file_path = path.join("timetable");
        if !file_path.exists() {
            let mut file = File::create(&file_path).expect("Could not create file");
            file.write(
                b"Ang Ang Mat Fis Fis Fiz Fiz\nBio Bio Eko Gju Gju Kim Kim\nEko Eko Gje Mat Bio Bio Fis\nKim Kim Fiz Fiz Let Mat Mat\nGje Let Let Mat Mat Ang ---\n",
            )
            .expect("Could not write to file");
        }
        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                if let Ok(ip) = line {
                    for word in ip.split_whitespace() {
                        timetable.push(word.to_string());
                    }
                }
            }
        }
    }

    let lookup_index = HashMap::from([
        (Weekday::Mon, 0),
        (Weekday::Tue, 1),
        (Weekday::Wed, 2),
        (Weekday::Thu, 3),
        (Weekday::Fri, 4),
        (Weekday::Sat, 5),
        (Weekday::Sun, 6),
    ]);

    let date = Local::now().date().weekday();

    for i in 0..DAYS {
        if i == lookup_index[&date] {
            print!("TODAY -> ")
        } else if i == lookup_index[&date] + 1 {
            print!("TOMOR -> ")
        } else {
            print!("-------- ")
        }

        for j in 0..HOURS {
            print!("{} ", timetable[j + i * HOURS])
        }
        println!();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
