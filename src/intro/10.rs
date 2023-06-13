fn solution(s1: String, s2: String) -> i32 {
    let mut common = [0; 26];

    for c in s1.chars() {
        common[c as usize - 'a' as usize] += 1;
    }

    let mut count = 0;
    for c in s2.chars() {
        if common[c as usize - 'a' as usize] > 0 {
            common[c as usize - 'a' as usize] -= 1;
            count += 1;
        }
    }

    count
}
