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
}
