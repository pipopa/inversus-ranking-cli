#[macro_use]
extern crate lazy_static;
extern crate reqwest;
use clap::{App, Arg};

mod ranking;

enum Mode {
    One,
    Two,
    Both,
}

fn main() {
    let app = App::new("inversus-rank")
        .version("1.0.0") // バージョン情報
        .author("pipopa <pipopa.710@gmail.com>") // 作者情報
        .about("CLI Inversus Ranking Viewer") // このアプリについて
        .arg(
            Arg::with_name("first") // フラグを定義
                .help("sample flag") // ヘルプメッセージ
                .short("f") // ショートコマンド
                .long("first"), // ロングコマンド
        )
        .arg(
            Arg::with_name("mode") // オプションを定義
                .help("arcade mode") // ヘルプメッセージ
                .short("m") // ショートコマンド
                .long("mode") // ロングコマンド
                .takes_value(true), // 値を持つことを定義
        );
    // 引数を解析
    let matches = app.get_matches();

    // optが指定されていれば値を表示
    let mode = if let Some(o) = matches.value_of("mode") {
        match o {
            "1" => Mode::One,
            "2" => Mode::Two,
            _ => Mode::Both,
        }
    } else {
        Mode::Both
    };

    if matches.is_present("first") {
        let ranking = match mode {
            Mode::One => ranking::fetch_players_one(true),
            Mode::Two => ranking::fetch_players_two(true),
            _ => ranking::fetch_1st_players(),
        };
        if let Ok(ranking) = ranking {
            println!("{:?}", ranking);
        } else {
            println!("Error!!");
        }
    }
}
