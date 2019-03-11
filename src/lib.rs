//! #Algorithms
//! A library for most common algorithms.

pub mod searching;
pub mod sorting;
pub mod data;


#[cfg(test)]
mod tests {
    #[test]
    fn quick_sort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        crate::sorting::quick_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        crate::sorting::quick_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
    #[test]
    fn bubble_sort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        crate::sorting::bubble_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        crate::sorting::bubble_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }

    #[test]
    fn merge_sort(){
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let result1=crate::sorting::merge_sort(&ve1);
        for i in 0..result1.len() - 1 {
            assert!(result1[i] <= result1[i + 1]);
        }

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let result2=crate::sorting::merge_sort(&ve2);
        for i in 0..result2.len() - 1 {
            assert!(result2[i] <=result2[i + 1]);
        }
    }
}

