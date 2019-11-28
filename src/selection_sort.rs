use std::vec::Vec;

pub fn sort(vec: &mut Vec<i32>) {
    let mut tmp = (0, 0);
    let mut i = 0;
    while i < vec.len() {
        tmp = (i, vec[i]);
        let mut j = i + 1;
        while j < vec.len() {
            let (min_order, value) = tmp;
            if value > vec[j] {
                tmp = (j, vec[j]);
            }
            j += 1;
        }
        let (min_order, value) = tmp;
        vec.swap(i, min_order);
        i += 1;
    }
}
