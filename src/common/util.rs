use std::cmp;
use std::iter;


const F32_ZERO: f32 = 1.0e-40_f32;


pub fn min_index<T: PartialOrd>(list: &Vec<T>) -> Option<usize> {
    let mut min_value = None;
    let mut min_index: usize = 0;

    let mut index: usize = 0;

    for i in list {
        min_value = match min_value {
            Some(v) => if i < v { min_index = index; Some(i) } else { Some(v) },
            None    => Some(i)
          };
        index += 1;
    }
    Some(min_index)
}

