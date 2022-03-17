use std::cell::Cell;

#[derive(Default)]
pub struct U32Reader {
    buf: Cell<Vec<u8>>,
}

impl U32Reader {
    /// Create a new reader.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads an unsigned integer from stdin that is known to end with a specific delimter.
    /// For example, use `b' '` for space-separated integers and `b'\n'` for integers separated by newlines.
    ///
    /// # Panics
    ///
    /// Panics when there are no more integers in stdin,
    /// when any I/O error occurs,
    /// and when the integer cannot be represented by a u32.
    #[must_use]
    pub fn read_until(&self, delim: u8) -> u32 {
        use std::io::{self, BufRead};

        // If any panic occurs below, the internal buffer becomes Vec::new().
        let mut buf = self.buf.take();

        // Clear the buffer for writing new data into it.
        buf.clear();

        let written = io::stdin()
            // Acquire a lock to stdin.
            .lock()
            // Read the decimal-encoded number into the buffer.
            .read_until(delim, &mut buf)
            // Panic if any I/O error occured.
            .expect("An I/O error occured when attempting to read from stdin");

        assert_ne!(written, 0, "No more integers can be read from stdin");

        // Convert the input into an integer.
        // `parse_partial`, unlike the standard library `parse`,
        // can ignore the delimiter(s) left by `read_until` at the end of the string.
        let result = lexical::parse_partial(&buf)
            .expect("Input cannot be represented by an unsigned 32-bit integer")
            .0;

        // Reuse the internal buffer for next read.
        self.buf.set(buf);

        result
    }

    /// Read one line of space-separated integers from stdin.
    ///
    /// # Panics
    ///
    /// See `read_until` for detailed semantics.
    pub fn read_line(&self, n: usize) -> impl Iterator<Item = u32> + '_ {
        (0..n - 1)
            // The first `n - 1` integers are terminated by a space character.
            .map(|_| self.read_until(b' '))
            // The last integer is terminated by a newline character (or EOF).
            .chain(std::iter::once_with(|| self.read_until(b'\n')))
    }
}
