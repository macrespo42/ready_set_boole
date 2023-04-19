fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut result:Vec<Vec<i32>> = Vec::new(); 
    let mut powerset_size: u32 = 2;
    powerset_size = powerset_size.pow(set.len() as u32);
    for counter in 0..powerset_size {
        let mut subset: Vec<i32> = Vec::new();
        for index in 0..set.len() {
            if counter & (1 << index) > 0 {
               subset.push(index as i32);
            }
        }
        result.push(subset);
    }
    result
}
