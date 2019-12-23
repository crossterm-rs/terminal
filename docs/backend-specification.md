# Supportability by Backend

| Backend | `Action` Not Supported |
| :------ | :------ |
| `termion` | ScrollUp, ScrollDown, SetTerminalSize, EnableBlinking, DisableBlinking |
| `crossterm` |   ...     |


| Backend | `Attribute` Not Supported |
| :------ | :------ |
| `termion` | ConcealOff, ConcealOff, Fraktur, NormalIntensity |
| `crossterm` | ...     | 

# Note by Backend

### Termion

- Fires thread to read input.
- Fires thread to capture terminal resize events.
- Uses stdout for terminal size 
- Uses `/dev/tty` and stdin for cursor position.

### Crossterm

- Uses stdout for cursor position.
