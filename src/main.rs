#![feature(non_ascii_idents)]

use rand::Rng;
use std::io::Write;

const 文字: &str = r##"0123456789QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~ "##;

fn main() -> !{
    println!("パスワード生成ウィザード passmaker 0.1.1");
    loop {
        print!("どの乱数を生成する?\n1.全ASCII文字 2.空白以外 3.記号以外 4.数字のみ\n> ");
        let _ = std::io::stdout().flush();
        let mut 入力 = String::new();
        let _ = std::io::stdin().read_line(&mut 入力);
        let 範囲;
        match 入力.trim() {
            "1" => 範囲 = 0..95,
            "2" => 範囲 = 0..94,
            "3" => 範囲 = 0..62,
            "4" => 範囲 = 0..10,
            _ => {
                eprintln!("エラー!");
                continue;
            }
        }
        print!("何文字生成する?\n> ");
        let _ = std::io::stdout().flush();
        let mut 入力 = String::new();
        let _ = std::io::stdin().read_line(&mut 入力);
        let 入力: usize = match 入力.trim().parse(){
            Ok(o) => o,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        let mut カウンタ: usize = 0;
        print!("パスワードは“");
        while 入力 > カウンタ {
            let 選択: usize = rand::rngs::OsRng.gen_range(範囲.clone());
            print!("{}", &文字[選択..選択 + 1]);
            カウンタ += 1;
        }
        print!("”です\n");
    }
}
