pub fn solution(a: Vec<i32>) -> Vec<i32> {
    let mut sorted = a.clone();

    sorted.sort();

    let mut result = Vec::new();

    let mut position = 0;

    loop {
        if sorted[position] == -1 && position < sorted.len() - 1 {
            position += 1;
        }
        else {
            break;
        }
    }

    for i in 0..a.len() {
        if a[i] == -1 {
            result.push(-1);
        }
        else {
            result.push(sorted[position]);
            position += 1;
        }
    }

    result
}
