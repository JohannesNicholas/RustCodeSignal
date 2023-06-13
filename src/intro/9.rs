fn solution(inputArray: Vec<String>) -> Vec<String> {
    let mut longest = 0;

    //loop through the array
    for i in 0..inputArray.len() {
        //if the current string is longer than the longest
        if inputArray[i].len() > longest {
            //set the longest to the current string's length
            longest = inputArray[i].len();
        }
    }

    //create a new vector
    let mut result = Vec::new();

    //loop through the array
    for i in 0..inputArray.len() {
        //if the current string is the same length as the longest
        if inputArray[i].len() == longest {
            //push the current string to the new vector
            result.push(inputArray[i].to_string());
        }
    }

    //return the new vector
    result
}
