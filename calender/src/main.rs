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
    Delete {
        /// 予定のID
        id: u64
    }
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
        Commands::Add { subject, start, end }
            => add_schedule(subject, start, end),
        Commands::Delete { id } => {
            let mut calender = read_calender();
            if delete_schedule(&mut calender, id) {
                save_calender(&calender);
                println!("予定を削除しました");
            } else {
                println!("エラー：IDが不正です");
            }
        }
    }
}

fn read_calender() -> Calender {
    let file = File::open(SCHEDULE_FILE).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn save_calender(calender: &Calender) {
    let file = File::create(SCHEDULE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, calender).unwrap()
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

fn delete_schedule(calender: &mut Calender, id: u64) -> bool {
    // 予定の削除
    for i in 0..calender.schedules.len() {
        if calender.schedules[i].id == id {
            calender.schedules.remove(i);
            return true;
        }
    }
    false
}

impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case(18, 15, 19, 15, true)]
    #[case(19, 45, 20, 45, true)]
    #[case(18, 30, 20, 15, true)]
    #[case(20, 15, 20, 45, true)]
    #[case(18, 15, 18, 45, true)]
    #[case(19, 15, 19, 45, true)]
    fn test_schedule_intersects(
        #[case]h0: u32,
        #[case]m0: u32,
        #[case]h1: u32,
        #[case]m1: u32,
        #[case]should_intersects: bool
    ) {
        let schedule = Schedule {
            id: 0,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, h0, m0, 0),
            end: naive_date_time(2024, 1, 1, h1, m1, 0),
        };
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, h0, m0, 0),
            end: naive_date_time(2024, 1, 1, h1, m1, 0),
        };
        assert_eq!(should_intersects, schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_delete_schedule() {
        let mut calender = Calender {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "テスト予定".to_string(),
                    start: naive_date_time(2023, 11, 19, 11, 22, 33),
                    end: naive_date_time(2023, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1,
                    subject: "テスト予定".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "追加テスト予定".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };
        assert!(delete_schedule(&mut calender, 0));

        let expected = Calender {
            schedules: vec![
                Schedule {
                    id: 1,
                    subject: "テスト予定".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "追加テスト予定".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };
        assert_eq!(expected, calender);
        // id=1の予定を削除してみる
        assert!(delete_schedule(&mut calender, 1));
        let expected = Calender {
            schedules: vec![
                Schedule {
                    id: 2,
                    subject: "追加テスト予定".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };
        assert_eq!(expected, calender);
        // id=2の予定を削除してみる
        assert!(delete_schedule(&mut calender, 2));
        let expected = Calender {
            schedules: vec![],
        };
        assert_eq!(expected, calender);
    }
}
