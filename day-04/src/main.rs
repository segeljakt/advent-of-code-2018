use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use scan_fmt::*;
//use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};


type Year  = i32;
type Month = i32;
type Day   = i32;
type Hour  = i32;
type Min   = i32;
type Kind  = String;
type Id    = i32;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Date {
    year: Year,
    month: Month,
    day: Day,
}

#[derive(Debug)]
struct Event {
    date:  Date,
    hour:  Hour,
    min:   Min,
    kind:  Kind
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let lines: std::str::Lines<'_> = buf.lines();

    let mut guards: HashMap<Id, HashMap<Date, [bool; 60]>> = HashMap::new();

    let mut events: Vec<Event> = lines
        .map(|line| {
            let res = scan_fmt!(
                line,
                "[{}-{}-{}_{}:{}]_{}",
                Year, Month, Day, Hour, Min, Kind
            );
            Event {
                date: Date {
                    year:  res.0.unwrap(),
                    month: res.1.unwrap(),
                    day:   res.2.unwrap(),
                },
                hour:  res.3.unwrap(),
                min:   res.4.unwrap(),
                kind:  res.5.unwrap(),
            }
        })
        .collect();

    events.sort_by(|a, b| {
        a.date.year.cmp(&b.date.year).then(
            a.date.month.cmp(&b.date.month).then(
                a.date.day.cmp(&b.date.day).then(
                    a.hour.cmp(&b.hour).then(
                        a.min.cmp(&b.min)
                    )
                )
            )
        )
    });

    let mut sleep_start = 0;
    let mut current_id = 0;

    for event in events {
        if let Some(id) = scan_fmt!(&event.kind, "Guard_#{}_begins_shift", Id) {
            current_id = id;
            guards.entry(current_id).or_insert(HashMap::new());
        } else if event.kind == "falls_asleep" {
            sleep_start = event.min;
        } else if event.kind == "wakes_up" {
            let ref mut guard = guards.get_mut(&current_id).unwrap();
            let ref mut shift = guard.entry(event.date).or_insert([false; 60]);
            let sleep_stop = event.min;
            for i in sleep_start..sleep_stop {
                shift[i as usize] = true;
            }
        } else {
            println!("{}", event.kind);
            unreachable!()
        }
    }

    let mut max_sleep = 0;
    let mut max_id = 0;

    for (id, shifts) in guards.iter() {
        let mut current_sleep = 0;
        for shift in shifts.values() {
            current_sleep += shift.iter().filter(|is_asleep| **is_asleep).count();
        }
        if max_sleep < current_sleep {
            max_id = *id;
            max_sleep = current_sleep;
        }
    }

    let mut minutes = [0; 60];
    for shift in guards.get(&max_id).unwrap().values() {
        for (is_asleep, minute) in shift.iter().zip(minutes.iter_mut()) {
            if *is_asleep {
                *minute += 1;
            }
        }
    }

    let max_minute = minutes
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|x| x.0)
        .unwrap();

    println!("Strategy 1: {} = {}*{}", max_id*(max_minute as i32), max_id, max_minute);

    let mut max_sleep = 0;
    let mut max_id = 0;
    let mut max_minute = 0;

    for (current_id, shifts) in guards.iter() {
        let mut minutes = [0; 60];
        for shift in shifts.values() {
            for (is_asleep, minute) in shift.iter().zip(minutes.iter_mut()) {
                if *is_asleep {
                    *minute += 1;
                }
            }
        }
        let (current_max_minute, current_max_sleep) = minutes
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        if max_sleep < *current_max_sleep {
            max_id = *current_id;
            max_sleep = *current_max_sleep;
            max_minute = current_max_minute;
        }
    }

    println!("Strategy 2: {} = {}*{}", max_id*(max_minute as i32), max_id, max_minute);
}
