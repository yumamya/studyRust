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
        Command::Deposit => unimplemented!(),
        Command::Withdraw => unimplemented!(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}

// fn new() {
//     // accounts.csvというファイル名で作成
//     let mut writer = Writer::from_path("accounts.csv").unwrap();
//     writer
//         .write_record(["日付", "用途", "金額"])
//         .unwrap();
//     writer.flush().unwrap();
// }