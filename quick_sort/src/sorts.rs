// Sort routines, for i32 vectors.

fn partion(vec: &mut [i32]) -> usize {
    let limit: usize = vec.len() - 1usize;
    let pivot: i32 = vec[limit];
    let mut pivot_point: usize = 0usize;

    for i in 0..limit {
        if vec[i] <= pivot {
            vec.swap(i, pivot_point);
            pivot_point += 1usize;
        }        
    };
    vec.swap(pivot_point, limit);
    pivot_point
}

pub(crate) fn quicksort(vec: &mut [i32]) {
    if vec.len() > 1usize {
        let pivot_point = partion(vec);

        quicksort(&mut vec[0..pivot_point]);
        quicksort(&mut vec[pivot_point+1..]);
    }
}