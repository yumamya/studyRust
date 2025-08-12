use rand::Rng;

fn main() {
    // 正解数を数える変数を追加
    let mut num_of_correct = 0;

    while num_of_correct < 3 {
        // 参考にしている技術書の記述ではバージョンが古い（0.8.5）なので、
        // 最新のrand（0.9.2）を利用しているとここがwarningになる
        // 下記の記述でwarning解消できる
        let op1 = rand::rng().random_range(0..100);
        let op2 = rand::rng().random_range(0..100);

        // 加算
        println!("{} + {} = ??", op1, op2);
        println!("?? の値を入力して下さい");
        let mut ans_input = String::new(); // ユーザからの回答を保持する変数
        // 標準入力から1行取得し、ans_input に代入する
        std::io::stdin().read_line(&mut ans_input).unwrap();

        // ans_input からtrim()で改行を取り除きparse()で整数（u32）型に変換する
        let ans_input = ans_input.trim().parse::<u32>().unwrap();

        dbg!(ans_input); // 実行後にキーボードで入力した値を確認できる
        if ans_input == op1 + op2 {
            println!("正解");

            num_of_correct += 1;

            if num_of_correct >= 3 {
                break;
            }
        } else {
            println!("不正解");
        }

        // 減算
        let op1 = rand::rng().random_range(0..100);
        let op2 = rand::rng().random_range(0..100);
        println!("{} - {} = ??", op1, op2);
        println!("?? の値を入力して下さい");
        let mut ans_input = String::new(); // ユーザからの回答を保持する変数
        // 標準入力から1行取得し、ans_input に代入する
        std::io::stdin().read_line(&mut ans_input).unwrap();

        // ans_input からtrim()で改行を取り除きparse()で符号付き整数（i32）型に変換する
        let ans_input = ans_input.trim().parse::<i32>().unwrap();

        dbg!(ans_input); // 実行後にキーボードで入力した値を確認できる
        if ans_input == op1 - op2 {
            println!("正解");

            num_of_correct += 1;

            if num_of_correct >= 3 {
                break;
            }
        } else {
            println!("不正解");
        }
    }

    println!("i32が扱えるデータ範囲： {} ~ {}", i32::MIN, i32::MAX);
    println!("u32が扱えるデータ範囲： {} ~ {}", u32::MIN, u32::MAX);
}
