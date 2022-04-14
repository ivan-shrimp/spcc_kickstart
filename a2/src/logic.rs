/// Returns `true` if the temperature `T` satisfies `36.0 <= T < 37.5`.
pub fn is_normal_temperature(temperature: f64) -> bool {
    (36.0..37.5).contains(&temperature)
}
