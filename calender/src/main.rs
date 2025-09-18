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
        // 予定の重複判定
    for schedule in &calender.schedules {
        if schedule.intersects(&new_schedule) {
            println!("エラー：予定が重複しています");
            return;
        }
    };
    // 予定の追加
    calender.schedules.push(new_schedule);

    // 予定の保存
    {
        let file = File::create(SCHEDULE_FILE).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &calender).unwrap();
    }
    println!("予定を追加しました");
}

impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_date_time(
        year: i32, 
        month: u32, 
        day: u32, 
        hour: u32, 
        minute: u32, 
        second: u32
    ) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    } 

    #[test]
    fn test_schedule_intersects_1() {
        // 2024/1/1 18:15から19:15までの既存予定1
        let schedule = Schedule {
            id: 1,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 18, 15, 0),
            end: naive_date_time(2024, 1, 1, 19, 15, 0),
        };

        // 2024/1/1 19:00から20:00までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };

        // 既存予定１と新規予定は重複している
        assert!(schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_schedule_intersects_2() {
        // 2024/1/1 19:45から20:45までの既存予定1
        let schedule = Schedule {
            id: 2,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 45, 0),
            end: naive_date_time(2024, 1, 1, 20, 45, 0),
        };

        // 2024/1/1 19:00から20:00までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };

        // 既存予定１と新規予定は重複している
        assert!(schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_schedule_intersects_3() {
        // 2024/1/1 18:30から20:15までの既存予定1
        let schedule = Schedule {
            id: 3,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 18, 30, 0),
            end: naive_date_time(2024, 1, 1, 20, 15, 0),
        };

        // 2024/1/1 19:00から20:00までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };

        // 既存予定１と新規予定は重複している
        assert!(schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_schedule_intersects_4() {
        // 2024/1/1 20:15から20:45までの既存予定1
        let schedule = Schedule {
            id: 4,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 20, 15, 0),
            end: naive_date_time(2024, 1, 1, 20, 45, 0),
        };

        // 2024/1/1 19:00から20:00までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };

        // 既存予定１と新規予定は重複しない
        assert!(!schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_schedule_intersects_5() {
        // 2023/12/8 9:00から10:30までの既存予定1
        let schedule = Schedule {
            id: 5,
            subject: "既存予定".to_string(),
            start: naive_date_time(2023, 12, 8, 9, 0, 0),
            end: naive_date_time(2023, 12, 8, 10, 30, 0),
        };

        // 2023/12/15 10:00から11:00までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2023, 12, 15, 10, 0, 0),
            end: naive_date_time(2023, 12, 15, 11, 0, 0),
        };

        // 既存予定１と新規予定は重複しない
        assert!(!schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_schedule_intersects_6() {
        // 2024/1/1 19:15から19:45までの既存予定1
        let schedule = Schedule {
            id: 6,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 15, 0),
            end: naive_date_time(2024, 1, 1, 19, 45, 0),
        };

        // 2024/1/1 19:00から20:00までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };

        // 既存予定１と新規予定は重複している
        assert!(schedule.intersects(&new_schedule));
    }
}
