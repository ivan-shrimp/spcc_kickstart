use std::io;

/// An adapter over a reader that reads `u32`s from it.
#[derive(Clone, Debug)]
pub struct U32Reader<R: io::BufRead> {
    buf: Vec<u8>,
    reader: R,
}

impl<R: io::BufRead> U32Reader<R> {
    /// Creates a new adapter that reads `u32`s from the given reader.
    #[must_use]
    pub const fn new(reader: R) -> Self {
        Self {
            buf: Vec::new(),
            reader,
        }
    }

    /// Returns a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Unwraps the adapter, returning the underlying reader.
    // This clippy lint is a false positive; compiler complains
    // "destructors cannot be evaluated at compile-time" if `const` is added.
    // This is because we have to drop the buffer.
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_inner(self) -> R {
        self.reader
    }

    /// Reads a `u32` from the underlying reader in a zero-copy manner.
    ///
    /// Use only when the delimiter is guaranteed to be a single byte,
    /// meaning that this is unsuitable for reading integers terminated by a newline,
    /// because that can end with either `\n` or `\r\n`.
    ///
    /// May return None if the underlying buffer of the reader does not contain any delimiter.
    /// One may use `read_until_fallback` in that case.
    ///
    /// Panics if no valid digits can be obtained from the underlying reader,
    /// if the integer cannot fit into a `u32`,
    /// or if any I/O error occured.
    #[must_use]
    fn read_until_fast(&mut self) -> Option<u32> {
        loop {
            // Get the internal buffer of the underying reader.
            let available = match self.reader.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => Err(e).expect("An input error occured"),
            };

            // Parse the first bytes of the buffer into an integer.
            // `parse_partial` returns both the parsed integer and, implicitly, the position of the delimiter.
            let (int, read) = lexical::parse_partial::<u32, _>(available)
                .expect("An error occured when parsing an input integer");

            // If any delimiter was found during parsing, return the parsed integer.
            // Otherwise, the integer might be split on the boundary between two buffers, so we should return None.
            return (read < available.len()).then(|| {
                // Consume the integer and the delimiter byte.
                self.reader.consume(read + 1);
                int
            });
        }
    }

    /// Reads a `u32` from the underlying reader
    /// by copying data from the reader into the internal buffer of this `U32Reader`.
    ///
    /// Can be used even if there are any other bytes between the digits and the delimiter,
    /// or if the digits are directly followed by EOF.
    /// For example, if the reader contains `123\r\n` and `delim` is set to `\n`, `123` will be correctly returned.
    ///
    /// Panics if no valid digits can be obtained from the underlying reader,
    /// if the integer cannot fit into a `u32`,
    /// or if any I/O error occured.
    #[must_use]
    fn read_until_fallback(&mut self, delim: u8) -> u32 {
        // Clear the internal buffer for writing new data into it.
        self.buf.clear();

        let used = self
            .reader
            // Read the decimal-encoded number into the buffer.
            .read_until(delim, &mut self.buf)
            .expect("An input error occured");

        assert_ne!(used, 0, "No more integers can be read");

        // Parse the input into an integer.
        // `parse_partial` ignores the delimiter(s) left by `read_until` at the end of the string.
        lexical::parse_partial(&self.buf)
            // Panic if any overflow occured.
            .expect("Input cannot be represented by an unsigned 32-bit integer")
            .0
    }

    /// Reads a `u32`, that is known to terminate with a space character (`' '`),
    /// from the underlying reader.
    ///
    /// Panics if no valid digits can be obtained from the underlying reader,
    /// if the integer cannot fit into a `u32`,
    /// or if any I/O error occured.
    #[must_use]
    pub fn read_until_space(&mut self) -> u32 {
        self.read_until_fast()
            .unwrap_or_else(|| self.read_until_fallback(b' '))
    }

    /// Reads a `u32`, that is known to terminate with a newline (`\n` or `\r\n`),
    /// from the underlying reader.
    ///
    /// Panics if no valid digits can be obtained from the underlying reader,
    /// if the integer cannot fit into a `u32`,
    /// or if any I/O error occured.
    #[must_use]
    pub fn read_until_newline(&mut self) -> u32 {
        self.read_until_fallback(b'\n')
    }
}
