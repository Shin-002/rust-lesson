use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("数を当ててみて!");

    // 乱数生成
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("予想した数を入力してみて!");

        // 予想の入力
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました。");

        // 入力された文字列を数値に変換（変換に失敗した場合はループの頭から再実行される）
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想: {}", guess);

        // 予想と当たりの比較（一致した場合はループ抜ける）
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます!"),
            Ordering::Greater => println!("大きすぎます!"),
            Ordering::Equal => {
                println!("当たり!");
                break;
            },
        }
    }
}
