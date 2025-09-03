use std::fs::OpenOptions;
use clap::{Args, Parser, Subcommand};
use csv::Writer;
#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作る
    New(NewArgs),
    /// 口座に入金
    Deposit,
    /// 口座から出金
    Withdraw,
    /// CSVからインポート
    Import,
    /// レポート出力
    Report,
}
#[derive(Args)]
struct NewArgs {
    account_name: String,
}
impl NewArgs {
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let mut writer = Writer::from_path(file_name).unwrap();
        writer.write_record(["日付", "用途", "金額"]).unwrap();
        writer.flush().unwrap();
    }
}
fn main() {
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit => deposit(),
        Command::Withdraw => withdraw(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}
fn deposit() {
    let open_option = OpenOptions::new()
        .write(true)
        .append(true)
        .open("口座1.csv")
        .unwrap();
    let mut writer = Writer::from_writer(open_option);
    writer.write_record(["1", "2", "3"]).unwrap();
    writer.flush().unwrap();
}
fn withdraw() {
    unimplemented!()
}