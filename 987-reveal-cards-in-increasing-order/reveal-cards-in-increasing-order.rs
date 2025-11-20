impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort();
        
    let mut result : Vec <i32> = vec![0; deck.len()];
    let mut index : usize = 0;
    let mut skip : bool = false;

    for & symbol in & deck 
    {
        loop 
        {
            index = index % deck.len();
            if result[index] == 0 
            {
                if skip
                {
                    skip = false;
                } 
                else 
                {
                    break;
                }
            }
            index += 1;
        }

        result[index] = symbol;
        skip = true;
    }

    return result;
    }
}