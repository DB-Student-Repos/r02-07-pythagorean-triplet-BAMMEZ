use std::collections::HashSet;
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    for a in 1..=(sum - 3) / 3 {
        for b in a + 1..=a + (sum - a * 3 - 1) / 2 {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                set.insert([a, b, c]);
            }
        }
    }
    set
}


 