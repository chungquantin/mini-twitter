pub fn average(numbers: Vec<u128>) -> f32 {
    numbers.iter().sum::<u128>() as f32 / numbers.len() as f32
}

pub fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}
