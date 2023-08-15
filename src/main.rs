#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use rand::Rng;
use std::io::Write;

const CHARACTERS: &str = r##"0123456789QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~ "##;

fn main() -> ! {
    println!("パスワード生成ウィザード passmaker 0.1.2");
    loop {
        print!("どの乱数を生成する?\n1.全ASCII文字 2.空白以外 3.記号以外 4.数字のみ\n> ");
        std::io::stdout().flush().unwrap();
        let mut mode = String::new();
        std::io::stdin().read_line(&mut mode).unwrap();
        let range = match mode.trim() {
            "1" => 0..95,
            "2" => 0..94,
            "3" => 0..62,
            "4" => 0..10,
            _ => {
                eprintln!("エラー!");
                continue;
            }
        };

        print!("何文字生成する?\n> ");
        std::io::stdout().flush().unwrap();
        let mut length = String::new();
        std::io::stdin().read_line(&mut length).unwrap();
        let length: usize = match length.trim().parse() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        print!("パスワードは“");
        for _ in 0..length {
            let index: usize = rand::rngs::OsRng.gen_range(range.clone());
            print!("{}", &CHARACTERS[index..index + 1]);
        }
        print!("”です\n");
    }
}
