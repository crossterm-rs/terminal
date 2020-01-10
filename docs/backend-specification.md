# Supportability by Backend

| Backend | `Action` Not Supported |
| :------ | :------ |
| `pancurses` | ScrollUp, ScrollDown, Enter/Leave alternate screen (default alternate screen) |
| `termion` | ScrollUp, ScrollDown,  |
| `crossterm` |       |


| Backend | `Attribute` Not Supported |
| :------ | :------ |
| `pancurses` | Fraktur, NormalIntensity, Framed |
| `termion` | ConcealOn, ConcealOff, Fraktur, NormalIntensity |
| `crossterm` |      | 

# Backend Evaluation

This section describes the pros and cons of each backend. 


### Crossterm

feature flag: (crossterm-backend)

**pros**
- Written in pure Rust
- Works crossplatform
- Performant
- Updates Regularly
- Supports all features of this library.
- Works without threads or spinning loops.
- Supports advanced event / modifier support.

**cons**
- Uses stdout for cursor position.

### Termion (termion-backend)

feature flag: (pancurses-backend)

**pros**
- Written in pure Rust
- Released as a marjor version crate
- Performant
- Supports Redox

**cons**
- Works on Unix systems only
- Uses threads for reading resize events and input
- Maintenance is limited
- Limited Modifier support.
- Fires thread to read input.
- Fires thread to capture terminal resize events.
- Uses stdout for terminal size 
- Uses `/dev/tty` and stdin for cursor position.

### Pancurses

feature flag: (pancurses-backend)

**pros**
- Based on ncurses and pdcurses. 
- Works crossplatform
- Supports advanced event / modifier support.

**cons**
- Depends on C ncurses library.
- Maintenance is limited
- Lacks some features (see above).
- Uses /dev/tty by default, falls back to stdout if not supported.
it is not possible to customize its buffer. Tough you do have full control over refreshing terminal screen.