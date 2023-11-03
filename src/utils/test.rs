pub fn get_random_length(min: i32, max: i32, rng: &mut quickcheck::Gen) -> i32 {
    let length_range = Vec::<i32>::from_iter(min..=max);
    let length = rng.choose(length_range.as_slice()).unwrap();
    return *length;
}