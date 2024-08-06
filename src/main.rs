use std::ops::{Add, Sub};
use std::{env, process};
use std::io;
use std::fs::{self};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::iter::zip;

fn get_timestamps(dir: &Path) -> io::Result<Vec<SystemTime>> {
    let mut timestamps: Vec<SystemTime> = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                timestamps.append(&mut get_timestamps(&path)?);
            } else {
                timestamps.push(entry.metadata()?.created()?);
                timestamps.push(entry.metadata()?.modified()?);
            }
        }
    }

    Ok(timestamps)
}

fn time_spent(timestamps: Vec<Duration>, inactivity_threshold_hours: u64) -> Duration {
    let mut timestamps_sorted: Vec<Duration> = timestamps.clone();
    timestamps_sorted.sort();

    let to_compare = zip(&timestamps_sorted[0 .. timestamps_sorted.len() - 1], &timestamps_sorted[1..]);
    to_compare.map(|(a, b)| b.sub(*a))
        .filter(|a| *a < Duration::new(3600 * inactivity_threshold_hours, 0))
        .fold(Duration::new(0,0), |sum, val| sum.add(val))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_count: usize = args.len();
    
    if arg_count == 1 {
        println!("No path specified, exiting...");
        process::exit(1);
    }

    let dir: &String = &args[1];
    let inactivity_threshold_hours: u64 = if arg_count > 2 {args[2].parse().unwrap_or(3)} else { 3 };

    println!("Using inactivity threshold of {} hours", inactivity_threshold_hours);
    println!("Looking through files in {}", dir);

    if let Ok(list) = get_timestamps(Path::new(dir)) {
        let durations: Vec<Duration> = list.iter().filter_map(|a| a.duration_since(UNIX_EPOCH).ok()).collect();
        let time_spent = time_spent(durations, inactivity_threshold_hours);
        let hours_spent = time_spent.as_secs_f64() / 3600.0;
        print!("Hours spent according to files: {:.2}", hours_spent);
    }
}

#[test]
fn active_interval() {
    let times: Vec<Duration> = vec![Duration::from_secs(1), Duration::from_secs(2), Duration::from_secs(3)];
    assert_eq!(Duration::from_secs(2), time_spent(times, 3));
}