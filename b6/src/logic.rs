/// Calculates the minimum distances that all students need to walk.
///
/// # Panics
///
/// Panics if there is no collection point among the classrooms.
pub fn minimum_distance(classrooms: impl IntoIterator<Item = bool>) -> u64 {
    use std::ops::ControlFlow;

    let mut next_classroom = {
        let mut classrooms = classrooms.into_iter();

        move || {
            classrooms.try_fold(
                0u32,
                |previous_empty_classrooms, this_is_collection_point| {
                    if this_is_collection_point {
                        // If we've reached a collection point,
                        // return the number of consecutive classrooms to the left that did not have a collection point.
                        ControlFlow::Break(previous_empty_classrooms)
                    } else {
                        // If we haven't reached a collection point,
                        // add one more empty classroom to the accumulator.
                        // (If the last classroom is reached,
                        //  the number of rightmost empty classrooms `x` will be returned as `ControlFlow::Continue(x)`.)
                        ControlFlow::Continue(previous_empty_classrooms + 1)
                    }
                },
            )
        }
    };

    let mut total_distance = match next_classroom() {
        // Initialize the total distance accumulator
        // with the distance sum of students to the left of the first collection point.
        //
        // Consider the following classrooms,
        // where there are 3 empty rooms to the left of X:
        //
        //             +---+---+---+---+----
        // classrooms  | 0 | 0 | 0 | 1 | ...
        //             +---+---+---+---+----
        //  distances    3>  2>  1>  X
        //
        // Here, the required distance sum is 1 + 2 + 3.
        // In general, it is 1 + 2 + ... + `empty_rooms`,
        // which is equal to `empty_rooms` * (`empty_rooms` + 1) / 2.
        ControlFlow::Break(empty_rooms) => {
            // Because `empty_rooms` can be around 5 * 10^5, the total distance can overflow a u32.
            let empty_rooms = u64::from(empty_rooms);
            empty_rooms * (empty_rooms + 1) / 2
        }
        ControlFlow::Continue(_) => panic!("There is no collection point among all classrooms"),
    };

    loop {
        match next_classroom() {
            // Consider the following classrooms,
            // where there are 6 empty rooms between X and Y:
            //
            //             ----+---+---+---+---+---+---+---+---+----
            // classrooms  ... | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ...
            //             ----+---+---+---+---+---+---+---+---+----
            //  distances        X  <1  <2  <3   3>  2>  1>  Y
            //
            // Here, the distance sum of students between X and Y is 1 + 2 + 3 + 3 + 2 + 1.
            // In general, it is 1 + 2 + ... + `empty_rooms` / 2 + `empty_rooms` / 2 + ... + 2 + 1,
            // which is equal to `empty_rooms` * (`empty_rooms` + 2) / 4.
            //
            // Now consider the following classrooms,
            // where there are 7 empty rooms between X and Y:
            //
            //             ----+---+---+---+---+---+---+---+---+---+----
            // classrooms  ... | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ...
            //             ----+---+---+---+---+---+---+---+---+---+----
            //  distances        X  <1  <2  <3  <4>  3>  2>  1>  Y
            //
            // Here, the required distance sum
            // The distance sum of students between X and Y is
            // 1 + 2 + ... + (`empty_rooms` + 1) / 2 + ... + 2 + 1,
            // which is equal to (`empty_rooms` + 1) * (`empty_rooms` + 1) / 4.
            //
            // From these two examples,
            // we notice that there is a difference between odd and even empty rooms.
            // However, notice that the expression for odd rooms can be rewritten as
            // "(`empty_rooms` * (`empty_rooms` + 2) / 4) + 1 / 4".
            // so using the odd-room formula for the even-room case
            // gives an answer that is 0.25 above the correct solution.
            // Therefore, we can rely on rounding down in integer division to
            // calculate the even-room case using the formula for the odd-room case.
            ControlFlow::Break(empty_rooms) => {
                let empty_rooms = u64::from(empty_rooms);
                total_distance += (empty_rooms + 1) * (empty_rooms + 1) / 4;
            }
            // We have reached the last classroom.
            // Add the distance sum for the rightmost students like the leftmost ones, and return.
            ControlFlow::Continue(final_empty_rooms) => {
                let final_empty_rooms = u64::from(final_empty_rooms);
                return total_distance + final_empty_rooms * (final_empty_rooms + 1) / 2;
            }
        }
    }
}
