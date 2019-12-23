# Contributing

I would appreciate any contributions to this crate. However, some things are handy to know.

## Code Style

### Import Order

All imports are semantically grouped and ordered. The order is:

- standard library (`use std::...`)
- external crates (`use rand::...`)
- current crate (`use crate::...`)
- parent module (`use super::..`)
- current module (`use self::...`)
- module declaration (`mod ...`)

There must be an empty line between groups. An example:

```rust
use crossterm_utils::{csi, write_cout, Result};

use crate::sys::{get_cursor_position, show_cursor};

use super::Cursor;
```

#### CLion Tips

The CLion IDE does this for you (_Menu_ -> _Code_ -> _Optimize Imports_). Be aware that the CLion sorts
imports in a group in a different way when compared to the `rustfmt`. It's effectively two steps operation
to get proper grouping & sorting:

* _Menu_ -> _Code_ -> _Optimize Imports_ - group & semantically order imports
* `cargo fmt` - fix ordering within the group

Second step can be automated via _CLion_ -> _Preferences_ ->
_Languages & Frameworks_ -> _Rust_ -> _Rustfmt_ -> _Run rustfmt on save_.  

### Max Line Length

| Type | Max line length |
| :--- | ---: |
| Code | 100 |
| Comments in the code | 120 |
| Documentation | 120 |

100 is the [`max_width`](https://github.com/rust-lang/rustfmt/blob/master/Configurations.md#max_width)
default value.

120 is because of the GitHub. The editor & viewer width there is +- 123 characters. 

### Warnings

The code must be warning free. It's quite hard to find an error if the build logs are polluted with warnings.
If you decide to silent a warning with (`#[allow(...)]`), please add a comment why it's required.

Always consult the [Travis CI](https://travis-ci.org/crossterm-rs/crossterm/pull_requests) build logs.

### Forbidden Warnings

Search for `#![deny(...)]` in the code:

* `unused_must_use`
* `unused_imports`

## Implementing Backend

1. Consider to create an issue for potential support. 
2. Add folder with the name of '{YOUR_BACKEND}' in /src/backend.
3. Add `mod.rs`, `implementation.rs` files to this folder. 
4. Create `BackendImpl` struct and implement `Backend` trait.       
    _maybe the code is out to date, then just implement the `Backend` trait._
    
    ```rust        
    pub struct BackendImpl<W: Write> {
        _phantom: PhantomData<W>,
    }
    ``` 

5. Implement Backend, check `/crossterm/implementation.rs` and `/termion/implementation.rs` out for references.

     ```rust
     use crate::{backend::Backend, error};
      
     impl<W: Write> Backend<W> for BackendImpl<W> {
         fn create() -> Self {
             unimplemented!()
         }
         
         fn act(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
             unimplemented!()
         }
         
         fn batch(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
             unimplemented!()
         }
         
         fn flush_batch(&mut self, buffer: &mut W) -> error::Result<()> {
             unimplemented!()
         }
         
         fn get(&self, retrieve_operation: Value) -> error::Result<()> {
             unimplemented!()
         }
     }
     ```
6. Reexport `{YOUR_BACKEND}::BackendImpl` in the module file you created at 3.
   `pub use self::implementation::BackendImpl;`.
   
7. Last but not least, export your module in `/src/backend/mod.rs`

```rust
#[cfg(feature = "your_backend")]
pub(crate) mod your_backend;

#[cfg(feature = "your_backend")]
pub(crate) use self::your_backend::BackendImpl;
```

8. Finaly, submit your PR.