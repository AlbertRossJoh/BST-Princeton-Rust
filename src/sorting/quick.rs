use crate::randomization;


/// Quicksort is a recusive algorithm that sorts an array by creating partitioning lines where elements before the line are guarenteed to be smaller than the values after.
/// The running time of this algorithm is *O(N log N)*, with a worst case of *O(N^2)* if the smallest and largest values are chosen as pivoting elements.
/// Select is also implemented and gets the value at position *K* as if it were sorted. Performance for this function is *O(N)* with a worst case of *O(N log N)*.
/// We can get a performance guarentee of both of these if the input array is shuffled. 
/// 
/// Author: AlberRossJoh
/// 
/// # Examples
/// ```
/// use itualgs_rs::sorting::quick::sort;
/// use itualgs_rs::sorting::quick::select;
///
/// let mut list = vec!['f','e','r','r','i','s'];
/// sort(&mut list);
/// 
/// assert_eq!(list, vec!['e','f','i','r','r','s']);
/// 
/// let mut new_list = vec!["does","the","bed","bugs","bite","?"];
/// 
/// let val = select(&mut new_list, 3);
/// 
/// assert_eq!(val, "bugs");
/// ```  


// Sorts a given vector using Quicksort
pub fn sort<T:Ord>(a: &mut Vec<T>) 
    where T:Clone
    {
    q_sort(a, 0, a.len()-1);
}

/// Shuffles the list before running quicksort
pub fn sort_shuffle<T:Ord>(a: &mut Vec<T>) 
    where T:Clone
    {
    randomization::lcg_random::shuffle_list(a);
    q_sort(a, 0, a.len()-1);
}

/// Shuffles the list before running quickselect
pub fn select_shuffle<T:Ord>(a: &mut Vec<T>, k:usize) -> T
    where T:Clone
    {
    randomization::lcg_random::shuffle_list(a);
    select(a, k)
}


fn q_sort<T:Ord>(a: &mut Vec<T>, lo:usize, hi:usize) 
    where T:Clone
    {
    if hi<=lo {
        return;
    }
    let j = partition(a, lo, hi);
    if j > 0 {
        q_sort(a, lo, j-1);
    }
    q_sort(a, j+1, hi);
}

// finds the k'th element of a unsorted vector nas if it was sorted
pub fn select<T:Ord>(a: &mut Vec<T>, k:usize) -> T 
    where T:Clone
    {
    
    let mut lo: usize = 0;
    let mut hi: usize = a.len()-1;

    while hi > lo {
        let i = partition(a, lo, hi);
        if i>k {
            hi = i - 1;
        } else if i<k {
            lo = i + 1;
        } else {
            return a[i].clone();
        }
    }
    a[lo].clone()
}


fn partition<T:Ord>(a: &mut Vec<T>, lo:usize, hi:usize) -> usize 
    where T: Clone
    {
    let mut i = lo.clone();
    let mut j = hi.clone()+1;

    let v = a[lo].clone();
    loop {
        loop {
            i += 1;
            if !less(&a[i], &v){
                break;
            }
            if i == hi {
                break;
            }
        }

        loop {
            j -= 1;
            if !less(&v, &a[j]){
                break;
            }
            if j == lo {
                break;
            }
        }
        if i>=j {
            break;
        }
        exch(a, i, j)
    }
    exch(a, lo, j);

    j
}

fn less<T:Ord>(v:&T, w:&T) -> bool{
    v.cmp(w).is_lt()
}

fn exch<T:Ord>(a: &mut Vec<T>, i:usize, j:usize){
    if i != j {
        a.swap(i, j);
    }
}

#[cfg(test)]
mod tests {

    use super::sort;
    use super::select;


    #[test]
    fn test_sort() {
        let mut list = vec![4,2,4,1];

        sort(&mut list);

        assert_eq!(list, vec![1,2,4,4]);
    }


    #[test]
    fn test_sort_char() {
        let mut list = vec!['f','e','r','r','i','s'];

        sort(&mut list);

        assert_eq!(list, vec!['e','f','i','r','r','s']);
    }


    #[test]
    fn test_sort_string() {
        let mut list = vec!["does","the","bed","bugs","bite","?"];

        sort(&mut list);

        assert_eq!(list, vec!["?","bed","bite","bugs","does","the"]);
    }

    #[test]
    fn test_select() {
        let mut list = vec!["does","the","bed","bugs","bite","?"];

        let val = select(&mut list, 3);

        assert_eq!(val, "bugs");
    }
}
