//https://app.codesignal.com/arcade/intro/level-3/9DgaPsE2a7M6M2Hu6 

fn solution(inputString: String) -> String {
    let mut s: Vec<char> = inputString.chars().collect();
    
    //a stack to keep track of the opening brackets
    let mut openings = Vec::new();

    //iterate through the string
    let mut i = 0;
    while i < s.len() {
        //if we find an opening bracket, push it to the stack
        if s[i] == '(' {
            openings.push(i);
        }
        //if we find a closing bracket, pop the last opening bracket from the stack
        else if s[i] == ')' {
            let opening = openings.pop().unwrap();
            //reverse the substring between the opening and closing brackets
            reverse_between(&mut s, opening, i);
            //remove the opening and closing brackets
            s.remove(i);
            s.remove(opening);
            //decrement i by 2 to account for the removed brackets
            i -= 2;
        }
        i += 1;
    }

    //return the string
    s.into_iter().collect()
}

fn reverse_between(s: &mut Vec<char>, left: usize, right: usize){
    let mut left = left;
    let mut right = right;
    while left < right {
        let temp = s[left];
        s[left] = s[right];
        s[right] = temp;
        left += 1;
        right -= 1;
    }
}