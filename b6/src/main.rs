fn main() {
    for classrooms in input_classrooms() {
        println!("{distance}", distance = b6::minimum_distance(classrooms));
    }
}

// Input routine.
fn input_classrooms() -> impl Iterator<Item = impl Iterator<Item = bool>> {
    use std::io::{self, Read};

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
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    // Panic if any unexpected I/O error occured.
                    Err(e) => panic!("An error occured when reading classroom data: {e}"),
                };
            }
        }
    }

    let u32_reader = read_u32::U32Reader::new();

    let test_case_count = u32_reader.read_until(b'\n');

    std::iter::repeat_with(move || {
        // First row is the number of classrooms, which we do not currently use.
        let _classroom_count = u32_reader.read_until(b'\n');

        // Second row is the classroom information, which we will create an iterator over.
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();

        ClassroomReader { inner: stdin_lock }
    })
    .take(test_case_count as usize)
}
