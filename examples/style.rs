use std::io::Write;

use bitflags::_core::time::Duration;
use std::thread;
use terminal::{error, stdout, Action, Clear, Color, TerminalLock};

fn draw_color_values_matrix_16x16<W, F>(
    w: &mut TerminalLock<W>,
    title: &str,
    color: F,
) -> error::Result<()>
where
    W: Write,
    F: Fn(u16, u16) -> Color,
{
    w.batch(Action::ClearTerminal(Clear::All))?;

    write!(w, "{}", title);
    w.flush();

    for idx in 0..=15 {
        w.batch(Action::MoveCursorTo(1, idx + 4))?;
        write!(w, "{}", format!("{:>width$}", idx, width = 2));

        w.batch(Action::MoveCursorTo(idx * 3 + 3, 3))?;
        write!(w, "{}", format!("{:>width$}", idx, width = 3));
    }

    for row in 0..=15u16 {
        w.batch(Action::MoveCursorTo(4, row + 4))?;

        for col in 0..=15u16 {
            w.batch(Action::SetForegroundColor(color(col, row)))?;
            write!(w, "███");
        }

        w.batch(Action::SetForegroundColor(Color::White))?;
        write!(w, "{}", format!("{:>width$} ..= ", row * 16, width = 3));
        write!(w, "{}", format!("{:>width$}", row * 16 + 15, width = 3));
    }

    w.flush_batch()?;

    Ok(())
}

fn rgb<W: Write>(lock: &mut TerminalLock<W>) {
    draw_color_values_matrix_16x16(lock, "Color::Rgb values", |col, row| {
        Color::AnsiValue((row * 16 + col) as u8)
    })
    .unwrap();
}

fn rgb_red_values<W: Write>(w: &mut TerminalLock<W>) -> error::Result<()> {
    draw_color_values_matrix_16x16(w, "Color::Rgb red values", |col, row| {
        Color::Rgb((row * 16 + col) as u8, 0 as u8, 0)
    })
}

fn rgb_green_values<W: Write>(w: &mut TerminalLock<W>) -> error::Result<()> {
    draw_color_values_matrix_16x16(w, "Color::Rgb green values", |col, row| {
        Color::Rgb(0, (row * 16 + col) as u8, 0)
    })
}

fn rgb_blue_values<W: Write>(w: &mut TerminalLock<W>) -> error::Result<()> {
    draw_color_values_matrix_16x16(w, "Color::Rgb blue values", |col, row| {
        Color::Rgb(0, 0, (row * 16 + col) as u8)
    })
}

fn main() {
    let terminal = stdout();
    let mut lock = terminal.lock_mut().unwrap();

    rgb(&mut lock);

    thread::sleep(Duration::from_millis(2000))
}
