pub fn linear_search<T: Eq>(arr: &[T], query: &T) -> Option<usize> {
    let mut result: Option<usize> = None;

    for (index, value) in arr.iter().enumerate() {
        if value == query {
            result = Some(index);
            break;
        }
    }

    result
}

/**
BINARY_SEARCH(arr, query)
    NOT_FOUND = -1
    low = 0
    high = arr.length() - 1
    mid = (high - low)/2

    while low <= high and arr[mid] != query:
        if arr[mid] < query:
            high = mid_index - 1
        else
            low = mid_index + 1

    if arr[mid] == query:
        return mid
    else:
        return NOT_FOUND
*/
pub fn binary_search<T: Ord>(arr: &[T], query: &T) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;
    let mut mid: usize = (high - low) / 2;

    while low < high && &arr[mid] != query {
        if &arr[mid] > query {
            high = mid - 1;
        } else {
            low = mid + 1;
        }

        mid = low + (high - low) / 2;
    }

    println!("\nfinal mid: {}", mid);
    if &arr[mid] == query {
        Some(mid)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn linear_search_should_retrieve_query_item() {
        let arr = [7, 1, 2, 4, 0, 3, 6, 22, 5];
        let query = 22;

        let result = linear_search(&arr, &query).unwrap();
        let expected = arr.len() - 2;

        assert_eq!(result, expected)
    }

    #[test]
    #[should_panic(expected = "No result")]
    fn linear_search_should_return_error_when_no_result() {
        let arr = [7, 1, 2, 4, 0, 3, 6, 22, 5];
        let query = 100;

        linear_search(&arr, &query).expect("No result");
    }

    #[test]
    fn binary_seach_best_case() {
        let sorted_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let query = 5;

        let result = binary_search(&sorted_arr, &query).unwrap();
        let expected = 4usize;

        assert_eq!(result, expected)
    }

    #[test]
    fn binary_search_worst_case_low() {
        let sorted_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let query = 1;

        let result = binary_search(&sorted_arr, &query).unwrap();
        let expected = 0usize;

        assert_eq!(result, expected)
    }

    #[test]
    fn binary_search_worst_case_high() {
        let sorted_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let query = 10;

        let result = binary_search(&sorted_arr, &query).unwrap();
        let expected = 9usize;

        assert_eq!(result, expected)
    }

    #[test]
    fn binary_search_non_worst_case() {
        let sorted_arr = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let query = 17;

        let result = binary_search(&sorted_arr, &query).unwrap();
        let expected = 16usize;

        assert_eq!(result, expected)
    }

    #[test]
    #[should_panic(expected = "No result")]
    fn binary_search_no_result() {
        let sorted_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let query = 11;

        let result = binary_search(&sorted_arr, &query).expect("No result");
        let expected = 9usize;

        assert_eq!(result, expected)
    }
}
