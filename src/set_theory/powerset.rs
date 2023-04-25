pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut result:Vec<Vec<i32>> = Vec::new(); 
    let mut cardinal: u32 = 2;
    cardinal = cardinal.pow(set.len() as u32);
    for subset_elt in 0..cardinal {
        let mut subset: Vec<i32> = Vec::new();
        for elt in 0..set.len() {
            if subset_elt & (1 << elt) > 0 {
               subset.push(set[elt]);
            }
        }
        result.push(subset);
    }
    result
}
