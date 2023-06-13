fn solution(n: i32) -> bool {
    let mut counter = 0;
    let length = n.to_string().len();
    let half = length / 2;

    for i in 0..half {
        let digit = n / 10_i32.pow(i as u32) % 10;
        counter += digit
    }

    for i in half..length {
        let digit = n / 10_i32.pow(i as u32) % 10;
        counter -= digit
    }

    return counter == 0;
}
