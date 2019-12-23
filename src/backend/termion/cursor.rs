use std::{io, io::Write};

use termion::get_tty;

use crate::error;
use std::io::BufRead;

/// Termion's cursor detections is terrible.
/// It panics a lot.
/// Although this solution it is not perfect, it works in most cases.
/// This can be used until it is fixed.
///
/// https://gitlab.redox-os.org/redox-os/termion/merge_requests/145
/// https://gitlab.redox-os.org/redox-os/termion/issues/173/
pub fn position() -> error::Result<(u16, u16)> {
    // Where is the cursor.unwrap()
    // Use `ESC [ 6 n`.
    let mut tty = get_tty().unwrap();
    let stdin = io::stdin();

    // Write command
    tty.write_all(b"\x1B[6n").unwrap();
    tty.flush().unwrap();

    stdin.lock().read_until(b'[', &mut vec![]).unwrap();

    let mut rows = vec![];
    stdin.lock().read_until(b';', &mut rows).unwrap();

    let mut cols = vec![];
    stdin.lock().read_until(b'R', &mut cols).unwrap();

    // remove delimiter
    rows.pop();
    cols.pop();

    let rows = String::from_utf8(rows).unwrap().parse::<u16>().unwrap();
    let cols = String::from_utf8(cols).unwrap().parse::<u16>().unwrap();

    Ok((cols - 1, rows - 1))
}
