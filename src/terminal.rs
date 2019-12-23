use std::{
    cell::RefCell,
    io::{self, Stderr, Stdout, Write},
    sync::{RwLock, RwLockWriteGuard},
};

use bitflags::_core::cell::RefMut;

use crate::{
    backend::{Backend as _, BackendImpl},
    error, Action, Retrieved, Value,
};

/// Creates a [Stdout](https://doc.rust-lang.org/std/io/struct.Stdout.html) buffered [Terminal](struct.Terminal.html).
pub fn stdout() -> Terminal<Stdout> {
    Terminal::custom(io::stdout())
}

/// Creates a [Stderr](https://doc.rust-lang.org/std/io/struct.Stdout.html) buffered [Terminal](struct.Terminal.html).
pub fn stderr() -> Terminal<Stderr> {
    Terminal::custom(io::stderr())
}

/// A simple interface to perform operations on the terminal.
/// It also allows terminal values to be queried.
///
/// # Examples
///
/// ```no_run
/// use terminal::{Clear, Action, Value, Retrieved, error};
///
/// pub fn main() -> error::Result<()> {
///     let terminal = terminal::stdout();
///
///     // perform an single action.
///     terminal.act(Action::ClearTerminal(Clear::All))?;
///
///     // batch multiple actions.
///     for i in 0..100 {
///         terminal.batch(Action::MoveCursorTo(0, i))?;
///     }
///
///     // execute batch.
///     terminal.flush_batch();
///
///     // get an terminal value.
///     if let Retrieved::TerminalSize(x, y) = terminal.get(Value::TerminalSize)? {
///         println!("x: {}, y: {}", x, y);
///     }
///
///     Ok(())
/// }
/// ```
///
/// # Notes
pub struct Terminal<W: Write> {
    // Access to the `Terminal` internals is ONLY allowed if this lock is acquired,
    // use `lock_mut()`.
    lock: RwLock<()>,
    // The internal buffer on which operations are performed and written to.
    buffer: RefCell<W>,
    // The selected backend implementation.
    backend: RefCell<BackendImpl<W>>,
}

impl<W: Write> Terminal<W> {
    /// Creates a custom buffered [Terminal](struct.Terminal.html) with the given buffer.
    pub fn custom(buffer: W) -> Terminal<W> {
        Terminal {
            lock: RwLock::new(()),
            backend: RefCell::new(BackendImpl::create()),
            buffer: RefCell::new(buffer),
        }
    }

    /// Locks this [Terminal](struct.Terminal.html), returning a mutable lock guard.
    /// A deadlock is not possible, instead an error will be returned if a lock is already in use.
    /// Make sure this lock is only used at one place.
    /// The lock is released when the returned lock goes out of scope.
    pub fn lock_mut(&self) -> error::Result<TerminalLock<'_, W>> {
        if let Ok(lock) = self.lock.try_write() {
            let backend = self.backend.borrow_mut();
            let buffer = self.buffer.borrow_mut();

            Ok(TerminalLock::new(lock, backend, buffer))
        } else {
            Err(error::ErrorKind::AttemptToAcquireLock(
                "`Terminal` can only be mutably borrowed once.".to_string(),
            ))
        }
    }

    /// Performs an action on the terminal.
    ///
    /// # Note
    ///
    /// Acquires an lock for underlying mutability,
    /// this can be prevented with [lock_mut](struct.Terminal.html#method.lock_mut).
    pub fn act(&self, action: Action) -> error::Result<()> {
        let mut lock = self.lock_mut()?;
        lock.act(action)
    }

    /// Batches an action for later execution.
    /// You can flush/execute the batched actions with [batch](struct.Terminal.html#method.flush_batch).
    ///
    /// # Note
    ///
    /// Acquires an lock for underlying mutability,
    /// this can be prevented with [lock_mut](struct.Terminal.html#method.lock_mut).
    pub fn batch(&self, action: Action) -> error::Result<()> {
        let mut lock = self.lock_mut()?;
        lock.batch(action)
    }

    /// Flushes the batched actions, this executes the actions in the order that they were batched.
    /// You can batch an action with [batch](struct.Terminal.html#method.batch).
    ///
    /// # Note
    ///
    /// Acquires an lock for underlying mutability,
    /// this can be prevented with [lock_mut](struct.Terminal.html#method.lock_mut).
    pub fn flush_batch(&self) -> error::Result<()> {
        let mut lock = self.lock_mut()?;
        lock.flush_batch()
    }

    /// Gets an value from the terminal.
    pub fn get(&self, value: Value) -> error::Result<Retrieved> {
        let lock = self.lock_mut()?;
        lock.get(value)
    }
}

impl<'a, W: Write> Write for Terminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut lock = self.lock_mut().unwrap();
        lock.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut lock = self.lock_mut().unwrap();
        lock.buffer.flush()
    }
}

/// A mutable lock to the [Terminal](struct.Terminal.html).
pub struct TerminalLock<'a, W: Write> {
    _lock: RwLockWriteGuard<'a, ()>,
    buffer: RefMut<'a, W>,
    backend: RefMut<'a, BackendImpl<W>>,
}

impl<'a, W: Write> TerminalLock<'a, W> {
    pub fn new(
        lock: RwLockWriteGuard<'a, ()>,
        backend: RefMut<'a, BackendImpl<W>>,
        buffer: RefMut<'a, W>,
    ) -> TerminalLock<'a, W> {
        TerminalLock {
            _lock: lock,
            buffer,
            backend,
        }
    }

    /// See [Terminal::act](struct.Terminal.html#method.act).
    pub fn act(&mut self, action: Action) -> error::Result<()> {
        self.backend.act(action, &mut self.buffer)
    }

    /// See [Terminal::batch](struct.Terminal.html#method.batch).
    pub fn batch(&mut self, action: Action) -> error::Result<()> {
        self.backend.batch(action, &mut self.buffer)
    }

    /// See [Terminal::flush_batch](struct.Terminal.html#method.flush_batch).
    pub fn flush_batch(&mut self) -> error::Result<()> {
        self.backend.flush_batch(&mut self.buffer)
    }

    /// See [Terminal::get](struct.Terminal.html#method.get).
    pub fn get(&self, value: Value) -> error::Result<Retrieved> {
        self.backend.get(value)
    }
}

impl<'a, W: Write> Write for TerminalLock<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

#[cfg(test)]
mod test {
    use crate::Terminal;
    use std::io::{Error, Write};

    struct BufferStub;

    impl Write for BufferStub {
        fn write(&mut self, _buf: &[u8]) -> Result<usize, Error> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn test_acquiring_mutable_lock_twice_should_error() {
        let stdout = Terminal::custom(BufferStub);

        let lock1 = stdout.lock_mut();
        let lock2 = stdout.lock_mut();

        assert!(lock1.is_ok());
        assert!(lock2.is_err());
    }
}
