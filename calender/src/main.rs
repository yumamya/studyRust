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
    /// 予定の追加
    Add {
        /// タイトル
        subject: String,
        /// 開始時刻
        start: NaiveDateTime,
        /// 終了時刻
        end: NaiveDateTime,
    },
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
        Commands::Add { subject, start, end }
            => add_schedule(subject, start, end),
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

fn add_schedule(
    subject: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
) {
    // 予定の読み込み
    let mut calender: Calender = {
        let file = File::open(SCHEDULE_FILE).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    // 予定の作成
    let id = calender.schedules.len() as u64;
    let new_schedule = Schedule {
        id, subject, start, end
    };
    // 予定の追加
    calender.schedules.push(new_schedule.clone());

    // 予定の重複判定
    for schedule in &calender.schedules {
        if schedule.start < new_schedule.end {
            println!("エラー：予定が重複しています");
            return;
        }
    }
    // 予定の保存
    {
        let file = File::create(SCHEDULE_FILE).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &calender).unwrap();
    }
    println!("予定を追加しました");
}
