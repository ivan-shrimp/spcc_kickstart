use std::io::prelude::*;

mod min_distance;

// for direct testing
pub use min_distance::minimum_distance;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    let mut test_cases = input_classrooms(input);

    while let Some(classrooms) = test_cases.next_test_case() {
        writeln!(
            output,
            "{distance}",
            distance = minimum_distance(classrooms)
        )
        .expect("An output error occured");
    }
}

#[derive(Debug)]
struct TestCaseReader<R> {
    inner: R,
    counter: u32,
}

impl<R: BufRead> TestCaseReader<R> {
    fn next_test_case(&mut self) -> Option<impl Iterator<Item = bool> + '_> {
        /// An iterator over the input test cases.
        #[derive(Debug)]
        struct ClassroomReader<R: Read> {
            inner: R,
        }

        impl<R: Read> Iterator for ClassroomReader<R> {
            type Item = bool;

            fn next(&mut self) -> Option<Self::Item> {
                let mut byte = 0;
                loop {
                    return match self.inner.read(std::slice::from_mut(&mut byte)) {
                        Ok(0) => None,
                        Ok(..) => match byte {
                            // '0' means there's no collection point at this classroom.
                            b'0' => Some(false),
                            // '1' means there's a collection point at this classroom.
                            b'1' => Some(true),
                            // We have reached the end of this test case.
                            b'\n' => None,
                            // This should be a part of a \r\n newline, so retry to get the \n.
                            b'\r' => continue,
                            // Panic if the input is invalid
                            // (or probably if the iterator is called again after yielding None).
                            _ => panic!("Invalid classroom data"),
                        },
                        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                        // Panic if any unexpected input error occured.
                        Err(e) => Err(e).expect("An input error occured"),
                    };
                }
            }
        }

        // Return `None` if there are no more test cases.
        self.counter = self.counter.checked_sub(1)?;

        // First row is the number of classrooms, which we do not currently use.
        let _classroom_count = read_u32::U32Reader::new(&mut self.inner).read_until_newline();

        Some(ClassroomReader {
            inner: &mut self.inner,
        })
    }
}

// Input routine.
fn input_classrooms(mut input: impl BufRead) -> TestCaseReader<impl BufRead> {
    let test_case_count = read_u32::U32Reader::new(&mut input).read_until_newline();
    TestCaseReader {
        inner: input,
        counter: test_case_count,
    }
}
