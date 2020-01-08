# Supportability by Backend

| Backend | `Action` Not Supported |
| :------ | :------ |
| `pancurses` | ScrollUp, ScrollDown, Enter/Leave alternate screen (default alternate screen) |
| `termion` | ScrollUp, ScrollDown,  |
| `crossterm` |   ...     |


| Backend | `Attribute` Not Supported |
| :------ | :------ |
| `pancurses` | Fraktur, NormalIntensity, Framed |
| `termion` | ConcealOn, ConcealOff, Fraktur, NormalIntensity |
| `crossterm` | ...     | 

# Note by Backend

### Termion

- Fires thread to read input.
- Fires thread to capture terminal resize events.
- Uses stdout for terminal size 
- Uses `/dev/tty` and stdin for cursor position.

### Crossterm

- Uses stdout for cursor position.

### Pancurses

- Uses /dev/tty by default, falls back to stdout if not supported.
it is not possible to customize its buffer. Tough you do have full control over refreshing terminal screen.
- 

# Benefits by each backend

This section describes the pros and cons of each backend. 

### Termion

**pros**
- Pure Rust
- Marjor version crate
- Performance
- Redox support

**cons**
- Unix Systems Only
- Uses threads for reading resize events and input
- Maintainence (see above)
- Modifier support.

### Crossterm

**pros**
- Pure Rust
- Crossplatform
- Performance
- Updated Regularly
- All features supported
- No threads, spinning loops.
- Advanced event / modifier support.

**cons**
TODO: Non-crossterm maintainer ;)

### Pancurses

**pros**
- Based on ncurses and pdcurses. 
- Crossplatform
- Advanced event / modifier support.

**cons**
- C dependency
- Maintainence
- Lacks some features (see above).