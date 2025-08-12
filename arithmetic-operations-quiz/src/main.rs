fn main() {
    println!("1 + 1 = ??");
    println!("?? の値を入力して下さい");
    let mut ans_input = String::new(); // ユーザからの回答を保持する変数
    // 標準入力から1行取得し、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input からtrim()で改行を取り除きparse()で整数（u32）型に変換する
    let ans_input = ans_input.trim().parse::<u32>().unwrap();

    dbg!(ans_input); // 実行後にキーボードで入力した値を確認できる
    if ans_input == 1 + 1 {
        println!("正解");
    } else {
        println!("不正解");
    }

    println!("1 - 4 = ??");
    println!("?? の値を入力して下さい");
    let mut ans_input = String::new(); // ユーザからの回答を保持する変数
    // 標準入力から1行取得し、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input からtrim()で改行を取り除きparse()で符号付き整数（i32）型に変換する
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input); // 実行後にキーボードで入力した値を確認できる
    if ans_input == 1 - 4 {
        println!("正解");
    } else {
        println!("不正解");
    }

    println!("i32が扱えるデータ範囲： {} ~ {}", i32::MIN, i32::MAX);
    println!("u32が扱えるデータ範囲： {} ~ {}", u32::MIN, u32::MAX);
}
