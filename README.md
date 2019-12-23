<h1 align="center"><img width="440" src="docs/terminal_full.png" /></h1>

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=Z8QK6XU749JB2) ![Travis][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] ![Lines of Code][s6] [![Join us on Discord][s5]][l5]

# Unified API for different TUI libraries.

This library offers a universal API over various terminal libraries such as 
[termion][termion], [crossterm][crossterm], [ncurses][ncurses], [pancurses][pancurses] and [console][console]. 

Why would I need this librarie? Three main reasons:

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
[dependencies.terminal-adapter]
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
terminal-adapter = "0.1"
features = ["your_backend_choice"] 
```

</details>
<p></p>
 
```rust
use terminal_adapter::{ClearType, Action, Value, Retreived, error};

pub fn main() -> error::Result<()> {
    let terminal = terminal_adapter::stdout();

    // perform an single action.
    terminal.act(Action::ClearTerminal(ClearType::All))?;

    // batch multiple actions.
    for i in 0..100 {
        terminal.batch(Action::MoveCursorTo(0, i))?;
    }

    // execute batch.
    terminal.flush_batch();

    // get an terminal-adapter value.
    if let Retreived::TerminalSize(x, y) = terminal.get(Value::TerminalSize)? {
        println!("x: {}, y: {}", x, y);
    }

    Ok(())
}
```

### Other Resources

- [API documentation](https://docs.rs/terminal-adapter/)
- [Examples repository](https://github.com/terminal-adapter/examples)
- [Backend Specification](docs/backend-specification.md)

## Contributing
  
I would appreciate any kind of contribution. Before you do, please,
read the [Contributing](docs/CONTRIBUTING.md) guidelines.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project, `terminal` are licensed under the MIT
License - see the [LICENSE](https://github.com/terminal-adapter/blob/master/LICENSE) file for details.

[s1]: https://img.shields.io/crates/v/terminal-adapter.svg
[l1]: https://crates.io/crates/terminal-adapter

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: terminal-adapter/LICENSE

[s3]: https://docs.rs/terminal-adapter/badge.svg
[l3]: https://docs.rs/terminal-adapter/

[s3]: https://docs.rs/terminal-adapter/badge.svg
[l3]: https://docs.rs/terminal-adapter/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s6]: https://tokei.rs/b1/github/terminal-adapter/?category=code
[s7]: https://travis-ci.org/terminal-adapter/.svg?branch=master

[termion]: https://crates.io/crates/termion
[crossterm]: https://crates.io/crates/crossterm
[cursive]: https://crates.io/crates/cursive
[tui]: https://crates.io/crates/tui
[termimad]: https://crates.io/crates/termimad
[ncurses]: https://crates.io/crates/ncurses
[pancurses]: https://crates.io/crates/pancurses
[console]: https://crates.io/crates/console