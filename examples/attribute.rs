#![allow(clippy::cognitive_complexity)]

use std::{io::Write, thread, time::Duration};
use terminal::{error::Result, stdout, Action, Attribute, TerminalLock};

const ATTRIBUTES: [(Attribute, Attribute); 7] = [
    (Attribute::Bold, Attribute::BoldOff),
    (Attribute::Italic, Attribute::ItalicOff),
    (Attribute::Underlined, Attribute::UnderlinedOff),
    (Attribute::Reversed, Attribute::ReversedOff),
    (Attribute::Crossed, Attribute::CrossedOff),
    (Attribute::SlowBlink, Attribute::BlinkOff),
    (Attribute::Conceal, Attribute::ConcealOff),
];

fn display_attributes<W: Write>(w: &mut TerminalLock<W>) -> Result<()> {
    let mut y = 2;
    w.write(b"Display attributes");

    for (on, off) in &ATTRIBUTES {
        w.act(Action::MoveCursorTo(0, y));

        w.batch(Action::SetAttribute(*on));
        w.write(format!("{:>width$} ", format!("{:?}", on), width = 35).as_bytes());
        w.batch(Action::SetAttribute(*off));
        w.write(format!("{:>width$}", format!("{:?}", off), width = 35).as_bytes());
        w.batch(Action::ResetColor);

        w.flush_batch();

        y += 1;
    }

    Ok(())
}

pub fn main() {
    let stdout = stdout();
    let mut lock = stdout.lock_mut().unwrap();

    display_attributes(&mut lock);

    thread::sleep(Duration::from_millis(5000))
}
