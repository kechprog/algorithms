pub fn qsort<T>(arr: &mut [T]) 
    where T: Ord + Copy
{
    match arr.len() {
        1 => { /* do nothing */ },
        2 => if arr[0] > arr[1] {
            (arr[1], arr[0]) = (arr[0], arr[1])
        }
        _ => {
            let mid_point = arr.len() / 2;
            let mid = arr[mid_point];
            let (mut lt, mut eq, mut gt) = (vec![], vec![], vec![]);

            arr.iter().for_each(|&e| {
                if e < mid {
                    lt.push(e);
                } else if e > mid {
                    gt.push(e);
                } else {
                    eq.push(e);
                }
            });

            qsort(&mut lt);
            qsort(&mut gt);

            let total_len = lt.len() + eq.len() + gt.len();
            arr[..lt.len()].copy_from_slice(&lt);
            arr[lt.len()..lt.len() + eq.len()].copy_from_slice(&eq);
            arr[lt.len() + eq.len()..total_len].copy_from_slice(&gt);
        }
    }
}
