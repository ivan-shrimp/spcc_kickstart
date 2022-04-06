use std::io::prelude::*;

mod validate;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    match input_sudoku_board(input) {
        Some(board) if validate::validate(&board) => {
            writeln!(output, "1")
        }
        _invalid => {
            writeln!(output, "0")
        }
    }
    .expect("An output error occured");
}

// Input routine.
// Returns a two-dimensional array containing numbers from 0 to 8.
//
// This part completes all validation required for Subtask 1.
fn input_sudoku_board(mut input: impl BufRead) -> Option<[[u8; 9]; 9]> {
    // An input buffer.
    let mut buf = Vec::new();

    // Read the first line into the buffer.
    input
        .read_until(b'\n', &mut buf)
        .expect("An input error occured");

    // The first line, which contains the width and height, must be "9 9" to be valid.
    match buf.as_slice() {
        b"9 9\n" | b"9 9\r\n" => {}
        _ => return None,
    }

    let mut result_array = [[0u8; 9]; 9];

    // Read the following 9 lines of 9 characters into the buffer.
    for row in &mut result_array {
        // Read one row into the buffer; it is guaranteed to have 9 characters.
        buf.clear();
        input
            .read_until(b'\n', &mut buf)
            .expect("An input error occured");

        // Check that the first 9 bytes are valid ASCII digits from '1' to '9'.
        for (cell, &byte) in std::iter::zip(row, &buf) {
            match byte {
                // Subtracting the digit by b'1' gives a number from 0 to 8 (corresponding to '1' to '9').
                digit @ b'1'..=b'9' => *cell = digit - b'1',
                _other => return None,
            }
        }

        // If there are any additional bytes, this is an incorrect row.
        match &buf[9..] {
            b"" | b"\n" | b"\r\n" => {}
            _additional => return None,
        }
    }

    Some(result_array)
}
