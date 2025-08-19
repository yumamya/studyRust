// 列挙型
#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

// 構造体
#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    // 上で定義した列挙型を使用
    suit: Suit,
    rank: i32,
}

use rand::seq::SliceRandom;
fn main() {
    // Vecの用意
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deckを作成
    for suit in suits {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }

    // Deckをシャッフル
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    // 手札用のVecの用意
    let mut hand: Vec<Card> = Vec::new();
    // 5枚のカードを引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札のソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札の表示
    println!("---Hand----");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    println!("入れ替えたいカードの番号を入力してください。（例：1 2 3）");
    // ユーザからの入力を入れるための変数
    let mut input = String::new();
    // ユーザから入力を変数に書き込む
    std::io::stdin().read_line(&mut input).unwrap();

    // 扱いやすいようにVecに変換する
    // （上から）
    // 文字列を空白区切りで分割（例　"1 2 3" => ["1", "2", "3"] ）
    // 文字列を数値に変換する（例　["1", "2", "3"] => [1, 2, 3]）
    // Vecに変換する
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    // 与えられた数字の箇所をデッキから取り出したカードに置き換える
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    // 手札のソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札の表示
    println!("---Hand----");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    // 役の判定（簡易）
    // フラッシュのチェック
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);
    // ペア数のチェック
    let mut count = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("フラッシュ");
    } else if count >= 3 {
        println!("スリーカード");
    } else if count == 2 {
        println!("2ペア");
    } else if count == 1 {
        println!("1ペア");
    } else {
        println!("役なし");
    }
}
