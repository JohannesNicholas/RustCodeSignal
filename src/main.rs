fn solution(sequence: Vec<i32>) -> bool {
    //remove the first local maxima
    let mut largest = sequence[0];
    for i in 1..sequence.len() {
        if sequence[i] > largest {
            largest = sequence[i];
        } else {
            //remove the element
            let mut copy = sequence.clone();
            copy.remove(i);
            let mut copy2 = sequence.clone();
            copy2.remove(i-1);
            return is_valid(sequence) || is_valid(sequence);
        }
    }
    return true;
}

fn is_valid(sequence: Vec<i32>) -> bool {
    for i in 1..sequence.len() {
        if sequence[i] <= sequence[i-1] {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("{}", solution(vec![3, 5, 67, 98, 3]));
}




// fn solution(sequence: Vec<i32>) -> bool {
//     let mut removed = false;
//     //loop through every element in the sequence
//     'outer: for i in 1..(sequence.len()-1) {
//         let a = sequence[i-1];
//         let b = sequence[i];
//         let c = sequence[i+1];
//         //kernel
//         if a < b && b < c {
//             continue 'outer;
//         }
//         if a < c {
//             if removed {
//                 return false;
//             }
//             removed = true;
//             continue 'outer;
//         }
//         return false;
//     }

//     return true;
// }





// fn solution(sequence: Vec<i32>) -> bool {
//     //loop through every element in the sequence
//     'outer: for i in 0..sequence.len() {
//         //create a copy of the sequence
//         //remove the current element from the copy

//         let bad_index = i;

//         //loop through the copy
//         if bad_index != 0 {
//             for j in 0..(bad_index-1) {
//                 //if the current element is greater than the next element
//                 //return false
//                 if sequence[j] >= sequence[j + 1] {
                    
//                     //return false;
//                     continue 'outer;
//                 }
//             }
//             if sequence.len() != bad_index+1 {
//                 if sequence[bad_index-1] >= sequence[bad_index+1] {
                    
//                     //return false;
//                     continue 'outer;
//                 }
//             }
            
//         }
        
//         for j in (bad_index+1)..(sequence.len()-1) {
//             //if the current element is greater than the next element
//             //return false
//             if sequence[j] >= sequence[j + 1] {
                
//                 //return false;
//                 continue 'outer;
//             }
//         }
//         return true;
//     }

//     return false;
// }
