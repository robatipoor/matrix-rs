use ansi_term::Colour::{Black, Blue, Green, White, Yellow};
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use termion::cursor;

fn main() {
    let chars = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', 'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'ｸ', 'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ',
        'ｿ', 'ﾀ', 'ﾁ', ' ', ' ', ' ', ' ', ' ', ' ', 'ﾂ', 'ﾃ', 'ﾄ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ', ' ', 'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ',
        'ﾏ', 'ﾐ', 'ﾑ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ',
        'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ', 'ﾜ', 'ﾝ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ];
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term)).unwrap();
    let colors = [Black, White, Green, Yellow, Blue];
    clear();
    let (row, col) = termion::terminal_size().unwrap();
    while !term.load(Ordering::Relaxed) {
        let i = rand::thread_rng().gen_range(1, row);
        let j = rand::thread_rng().gen_range(1, col);
        print!(
            "{}{}",
            colors[random::<usize>() % colors.len()]
                .paint(chars[random::<usize>() % chars.len()].to_string()),
            cursor::Goto(i, j),
        );
        sleep(Duration::from_millis(3));
    }
    clear()
}

fn clear() {
    println!("{}", termion::clear::All);
    println!("{}", cursor::Goto(1, 1))
}
