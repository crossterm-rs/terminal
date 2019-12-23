use std::io::Write;

use terminal_adapter::{error, stdout, Action, ClearType, Color, TerminalLock};

fn test_color_values_matrix_16x16<W, F>(
    w: &mut TerminalLock<W>,
    title: &str,
    color: F,
) -> error::Result<()>
where
    W: Write,
    F: Fn(u16, u16) -> Color,
{
    w.batch(Action::ClearTerminal(ClearType::All));

    print!("{}", title);

    for idx in 0..=15 {
        w.batch(Action::MoveCursorTo(1, idx + 4));
        print!("{}", format!("{:>width$}", idx, width = 2));

        w.batch(Action::MoveCursorTo(idx * 3 + 3, 3));
        print!("{}", format!("{:>width$}", idx, width = 3));
    }

    for row in 0..=15u16 {
        w.batch(Action::MoveCursorTo(4, row + 4));

        for col in 0..=15u16 {
            w.batch(Action::SetForegroundColor(color(col, row)));
            print!("███");
        }

        w.batch(Action::SetForegroundColor(Color::White));
        print!("{}", format!("{:>width$} ..= ", row * 16, width = 3));
        print!("{}", format!("{:>width$}", row * 16 + 15, width = 3));
    }

    w.flush_batch()?;

    Ok(())
}

fn rgb<W: Write>(lock: &mut TerminalLock<W>) {
    test_color_values_matrix_16x16(lock, "Color::Rgb green values", |col, row| {
        Color::AnsiValue((row * 16 + col) as u8)
    })
    .unwrap();
}

fn rgb_red_values<W: Write>(w: &mut TerminalLock<W>) -> error::Result<()> {
    test_color_values_matrix_16x16(w, "Color::Rgb red values", |col, row| {
        Color::Rgb((row * 16 + col) as u8, 0 as u8, 0)
    })
}

fn rgb_green_values<W: Write>(w: &mut TerminalLock<W>) -> error::Result<()> {
    test_color_values_matrix_16x16(w, "Color::Rgb green values", |col, row| {
        Color::Rgb(0, (row * 16 + col) as u8, 0)
    })
}

fn rgb_blue_values<W: Write>(w: &mut TerminalLock<W>) -> error::Result<()> {
    test_color_values_matrix_16x16(w, "Color::Rgb blue values", |col, row| {
        Color::Rgb(0, 0, (row * 16 + col) as u8)
    })
}

fn main() {
    let stdout = stdout();
    let mut lock = stdout.lock_mut().unwrap();

    rgb(&mut lock);
}
