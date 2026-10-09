use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERMINAL_BUFFER: Mutex<String> = Mutex::new(String::new());
}

pub fn print_str(s: &str) {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    {
        let mut b = TERMINAL_BUFFER.lock().unwrap();
        b.push_str(s);
    }
}

pub fn println_str(s: &str) {
    {
        let mut b = TERMINAL_BUFFER.lock().unwrap();
        b.push_str(s);
        b.push('\n');
    }
}

pub fn flush_buffer() -> String {
    {
        let mut b = TERMINAL_BUFFER.lock().unwrap();
        let res = b.clone();
        b.clear();
        res
    }
}

pub const C_RESET: &str = "\x1b[0m";
pub const C_BOLD: &str = "\x1b[1m";
pub const C_DIM: &str = "\x1b[2m";
pub const C_RED: &str = "\x1b[31m";
pub const C_GREEN: &str = "\x1b[32m";
pub const C_YELLOW: &str = "\x1b[33m";
pub const C_BLUE: &str = "\x1b[34m";
pub const C_MAGENTA: &str = "\x1b[35m";
pub const C_CYAN: &str = "\x1b[36m";
pub const C_WHITE: &str = "\x1b[37m";
pub const C_BRED: &str = "\x1b[91m";
pub const C_BGREEN: &str = "\x1b[92m";
pub const C_BYELLOW: &str = "\x1b[93m";
pub const C_BBLUE: &str = "\x1b[94m";
pub const C_BMAGENTA: &str = "\x1b[95m";
pub const C_BCYAN: &str = "\x1b[96m";
pub const C_BWHITE: &str = "\x1b[97m";

pub const BOX_W: usize = 52;

pub fn ui_top() {
    print_str("    ");
    print_str(C_CYAN);
    for _ in 0..BOX_W {
        print_str("=");
    }
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_div() {
    print_str("    ");
    print_str(C_CYAN);
    for _ in 0..BOX_W {
        print_str("-");
    }
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_bot() {
    ui_top();
}

pub fn ui_blank() {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_title(text: &str) {
    let plain_text_len = text
        .replace(C_RESET, "")
        .replace(C_BOLD, "")
        .replace(C_DIM, "")
        .replace(C_RED, "")
        .replace(C_GREEN, "")
        .replace(C_YELLOW, "")
        .replace(C_BLUE, "")
        .replace(C_MAGENTA, "")
        .replace(C_CYAN, "")
        .replace(C_WHITE, "")
        .replace(C_BRED, "")
        .replace(C_BGREEN, "")
        .replace(C_BYELLOW, "")
        .replace(C_BBLUE, "")
        .replace(C_BMAGENTA, "")
        .replace(C_BCYAN, "")
        .replace(C_BWHITE, "")
        .len();

    let pad = if BOX_W > 2 + plain_text_len {
        (BOX_W - 2 - plain_text_len) / 2
    } else {
        0
    };

    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    for _ in 0..pad {
        print_str(" ");
    }
    print_str(C_BOLD);
    print_str(C_BYELLOW);
    print_str(text);
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_row(text: &str) {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    print_str("  ");
    print_str(text);
    print_str("\n");
}

pub fn ui_row2(text: &str) {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden in UI components)
    print_str("    ");
    print_str(text);
    print_str("\n");
}

pub fn ui_raw() {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    print_str("  ");
}

pub fn ui_prompt(text: &str) {
    print_str("\n    ");
    print_str(C_BCYAN);
    print_str(">> ");
    print_str(C_BWHITE);
    print_str(text);
    print_str(C_RESET);
}

pub fn ui_win(text: &str) {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    print_str("  ");
    print_str(C_BGREEN);
    print_str("** ");
    print_str(text);
    print_str(" **");
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_lose(text: &str) {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    print_str("  ");
    print_str(C_RED);
    print_str(text);
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_push(text: &str) {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    print_str("  ");
    print_str(C_YELLOW);
    print_str(text);
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_info(text: &str) {
    print_str("    ");
    print_str(C_CYAN);
    print_str("|");
    print_str(C_RESET);
    print_str("  ");
    print_str(C_DIM);
    print_str(text);
    print_str(C_RESET);
    print_str("\n");
}

pub fn ui_cards(
    rank_strs: &[&str],
    suit_strs: &[&str],
    colors: &[&str],
    n: usize,
    hide_idx: Option<usize>,
) {
    // Top border
    print_str("    "); print_str(C_CYAN); print_str("|"); print_str(C_RESET); print_str("    ");
    for i in 0..n {
        if Some(i) == hide_idx {
            print_str(C_DIM); print_str("+--------+"); print_str(C_RESET);
        } else {
            print_str(colors[i]); print_str("+--------+"); print_str(C_RESET);
        }
        if i < n - 1 { print_str("  "); }
    }
    print_str("\n");

    // Line 1: Top rank
    print_str("    "); print_str(C_CYAN); print_str("|"); print_str(C_RESET); print_str("    ");
    for i in 0..n {
        if Some(i) == hide_idx {
            print_str(C_DIM); print_str("| ****** |"); print_str(C_RESET);
        } else {
            print_str(colors[i]); print_str("| ");
            let rank = rank_strs[i];
            print_str(rank);
            let pad = 6 - rank.chars().count();
            for _ in 0..pad { print_str(" "); }
            print_str(" |"); print_str(C_RESET);
        }
        if i < n - 1 { print_str("  "); }
    }
    print_str("\n");

    // Line 2: Empty
    print_str("    "); print_str(C_CYAN); print_str("|"); print_str(C_RESET); print_str("    ");
    for i in 0..n {
        if Some(i) == hide_idx {
            print_str(C_DIM); print_str("| ****** |"); print_str(C_RESET);
        } else {
            print_str(colors[i]); print_str("|        |"); print_str(C_RESET);
        }
        if i < n - 1 { print_str("  "); }
    }
    print_str("\n");

    // Line 3: Center Suit
    print_str("    "); print_str(C_CYAN); print_str("|"); print_str(C_RESET); print_str("    ");
    for i in 0..n {
        if Some(i) == hide_idx {
            print_str(C_DIM); print_str("| **??** |"); print_str(C_RESET);
        } else {
            print_str(colors[i]); print_str("|   ");
            let suit = suit_strs[i];
            print_str(suit);
            let pad = 4 - suit.chars().count();
            for _ in 0..pad { print_str(" "); }
            print_str(" |"); print_str(C_RESET);
        }
        if i < n - 1 { print_str("  "); }
    }
    print_str("\n");

    // Line 4: Empty
    print_str("    "); print_str(C_CYAN); print_str("|"); print_str(C_RESET); print_str("    ");
    for i in 0..n {
        if Some(i) == hide_idx {
            print_str(C_DIM); print_str("| ****** |"); print_str(C_RESET);
        } else {
            print_str(colors[i]); print_str("|        |"); print_str(C_RESET);
        }
        if i < n - 1 { print_str("  "); }
    }
    print_str("\n");

    // Line 5: Bottom rank
    print_str("    "); print_str(C_CYAN); print_str("|"); print_str(C_RESET); print_str("    ");
    for i in 0..n {
        if Some(i) == hide_idx {
            print_str(C_DIM); print_str("| ****** |"); print_str(C_RESET);
        } else {
            print_str(colors[i]); print_str("| ");
            let rank = rank_strs[i];
            let pad = 6 - rank.chars().count();
            for _ in 0..pad { print_str(" "); }
            print_str(rank);
            print_str(" |"); print_str(C_RESET);
        }
        if i < n - 1 { print_str("  "); }
    }
    print_str("\n");

    // Bottom border
    print_str("    "); print_str(C_CYAN); print_str("|"); print_str(C_RESET); print_str("    ");
    for i in 0..n {
        if Some(i) == hide_idx {
            print_str(C_DIM); print_str("+--------+"); print_str(C_RESET);
        } else {
            print_str(colors[i]); print_str("+--------+"); print_str(C_RESET);
        }
        if i < n - 1 { print_str("  "); }
    }
    print_str("\n");
}

pub fn ui_suit_color(suit: i32) -> &'static str {
    match suit {
        1 | 2 => C_BRED, // Diamonds or Hearts
        _ => C_BWHITE,
    }
}
