use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use std::{fs::File, io::{BufReader, BufWriter}};
use clap::{Parser, Subcommand};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    // 予定ID
    id: u64,
    // 勉強会名
    subject: String,
    // 開始時刻
    start:NaiveDateTime,
    // 終了時刻
    end: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Calender {
    schedules: Vec<Schedule>,
}

const SCHEDULE_FILE : &str = "schedule.json";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 予定一覧表示
    List,
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
    }
}

fn show_list() {
    // 読み込み
    let calender : Calender = {
        let file = File::open(SCHEDULE_FILE).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };
    // 予定表示
    println!("ID\tSTART\tEND\tSUBJECT");
    for schedule in calender.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        );
    }
}
