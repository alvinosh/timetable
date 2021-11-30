use chrono::prelude::*;
use core::time;
use std::collections::HashMap;

fn main() {
    let timetable = [
        ["Ang", "Ang", "Mat", "Fis", "Fis", "Fiz", "Fiz"],
        ["Bio", "Bio", "Eko", "Gju", "Gju", "Kim", "Kim"],
        ["Eko", "Eko", "Gje", "Mat", "Bio", "Bio", "Fis"],
        ["Kim", "Kim", "Fiz", "Fiz", "Let", "Mat", "Mat"],
        ["Gje", "Let", "Let", "Mat", "Mat", "Ang", ""],
    ];

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

    for (i, day) in timetable.iter().enumerate() {
        if i == lookup_index[&date] {
            print!("TODAY -> ")
        } else if i == lookup_index[&date] + 1 {
            print!("TOMOR -> ")
        } else {
            print!("-------- ")
        }

        for hour in day {
            print!("{} ", hour)
        }
        println!();
    }
}
