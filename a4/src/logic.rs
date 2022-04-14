/// Returns the minimum number of steps to reach the lab.
pub fn min_steps(staircase_size: u32) -> u32 {
    // Our way to climb the staircase is to climb 3 stairs at a time,
    // then if 1 or 2 stairs remain, climb the remaining stairs in one step.
    let triple_stair_steps = staircase_size / 3;
    let stairs_completed = triple_stair_steps * 3;
    triple_stair_steps + u32::from(stairs_completed != staircase_size)
}
