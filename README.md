<h1 align="center"><img width="550" src="docs/terminal_full.png" /></h1>

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=Z8QK6XU749JB2) 
[![Latest Version][crate-badge]][crate-link] 
[![docs][docs-badge]][docs-link]
![Lines of Code][loc-badge]
[![MIT][license-badge]][license-link] 
[![Join us on Discord][discord-badge]][discord-link]
[![Stable Status][actions-stable-badge]][actions-link]
[![Beta Status][actions-nightly-badge]][actions-link]

# Unified API for different TUI libraries.

This library offers a universal API over various terminal libraries such as 
[termion][termion], [crossterm][crossterm], [ncurses][ncurses], [pancurses][pancurses] and [console][console]. 

Why would I need this library? Three main reasons:

1) These libraries differ in the API.
 
    A smart choice would be to create an adapter layer to one of these libraries so that you wont have an direct dependency 
    and you won't need to update your code base when you want to switch or upgrade. Creating those adapters is boring (mapping types).
    Fortunately, this library does that for you. Some examples of those mappings can be found in those libraries: ([cursive][cursive], [tui][tui], [termimad][termimad], ...).
2) These libraries can be complex for beginners. 
 
    This library offers a very thin and simple abstraction to make it somewhat easier for the user.
    This is achieved by hiding the implementation details. 
    Implementation details cover raw mode, write to buffer, batch operations.

3) Libraries differ in how they work. 

    Like cursor 0 or 1 based, cleaning resources, event handling, performing actions.  


## Table of Contents

* [Features](#features)
* [Implemented Backends](#implemented-backends)
* [Getting Started](#getting-started)
* [Other Resources](#other-resources)
* [Contributing](#contributing)

## Features

- Batching multiple terminal commands before executing (flush).
- Complete control over the underlying buffer.
- Locking the terminal for a certain duration.
- Backend of your choice.

    
<!--
WARNING: Do not change following heading title as it's used in the URL by other crates!
-->

### Implemented Backends

- [Crossterm][crossterm] (Pure rust and crossplatform)
- [Termion][termion] (Pure rust for UNIX systems)


Use **one** of the below feature flags to choose an backend. 


| Feature | Description |
| :------ | :------ |
| `crossterm-backend` | crossterm backend will be used.|
| `termion-backend` | termion backend will be used.|

_like_
```toml
[dependencies.terminal]
version = "0.1"
features = ["crossterm-backend"] 
```

### Yet to Implement
- [ncurses][ncurses]
- [pancurses][pancurses]

## Getting Started

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
terminal = "0.1"
features = ["your_backend_choice"] 
```

</details>
<p></p>
 
```rust
use terminal::{ClearType, Action, Value, Retreived, error};

pub fn main() -> error::Result<()> {
    let terminal = terminal::stdout();

    // perform an single action.
    terminal.act(Action::ClearTerminal(ClearType::All))?;

    // batch multiple actions.
    for i in 0..100 {
        terminal.batch(Action::MoveCursorTo(0, i))?;
    }

    // execute batch.
    terminal.flush_batch();

    // get an terminal value.
    if let Retreived::TerminalSize(x, y) = terminal.get(Value::TerminalSize)? {
        println!("x: {}, y: {}", x, y);
    }

    Ok(())
}
```

### Other Resources

- [API documentation](https://docs.rs/terminal/)
- [Examples repository](https://github.com/crossterm-rs/terminal/examples)
- [Backend Specification](docs/backend-specification.md)

## Contributing
  
I would appreciate any kind of contribution. Before you do, please,
read the [Contributing](docs/CONTRIBUTING.md) guidelines.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project, `terminal` are licensed under the MIT
License - see the [LICENSE](https://github.com/crossterm-rs/terminal/blob/master/LICENSE) file for details.

[crate-badge]: https://img.shields.io/crates/v/terminal.svg
[crate-link]: https://crates.io/crates/terminal

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: terminal/LICENSE

[docs-badge]: https://docs.rs/terminal/badge.svg
[docs-link]: https://docs.rs/terminal/

[discord-badge]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[discord-link]: https://discord.gg/K4nyTDB

[actions-link]: https://github.com/crossterm-rs/terminal/actions
[actions-stable-badge]: https://github.com/crossterm-rs/terminal/workflows/Terminal%20Adapter%20Test/badge.svg
[actions-nightly-badge]: https://github.com/crossterm-rs/terminal/workflows/Terminal%20Adapter%20Test/badge.svg

[loc-badge]: https://tokei.rs/b1/github/crossterm-rs/terminal?category=code

[termion]: https://crates.io/crates/termion
[crossterm]: https://crates.io/crates/crossterm
[cursive]: https://crates.io/crates/cursive
[tui]: https://crates.io/crates/tui
[termimad]: https://crates.io/crates/termimad
[ncurses]: https://crates.io/crates/ncurses
[pancurses]: https://crates.io/crates/pancurses
[console]: https://crates.io/crates/console
