fn solution(matrix: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let mut haunted: Vec<bool> = vec![false; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let val = matrix[i][j];
            
            if val != 0 && !haunted[j] {
                sum += val;
            } else {
                haunted[j] = true;
            }
        }
    }
    sum
}


fn main() {
    println!("{}", solution(vec![3, 5, 67, 98, 3]));
}
