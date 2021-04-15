use std::{cmp, collections::HashMap, iter::FromIterator};

// -------------------- Add two binary numbers of n bits --------------------

pub fn add_binary(l: &[bool], r: &[bool]) -> Vec<bool> {
  let mut result = vec![false; l.len() + 1];

  for i in (0..l.len()).rev() {
    let carry_in = result[i + 1];

    let mut bit_result = false;
    let mut carry_out = false;
    let mut on_count: u8 = 0;

    for bit in [carry_in, l[i], r[i]].iter() {
      if *bit {
        on_count += 1
      }
    }

    if on_count >= 2 {
      carry_out = true;
    }
    if on_count == 1 || on_count == 3 {
      bit_result = true;
    }

    result[i + 1] = bit_result;
    result[i] = carry_out;
  }

  result
}

// ------- https://www.hackerrank.com/challenges/sock-merchant/problem ------

pub fn sock_merchant(ar: &[u32]) -> u32 {
  let mut sock_hash: HashMap<&u32, u32> = HashMap::new();
  let mut pair_count = 0u32;

  ar.iter().for_each(|color| {
    if sock_hash.contains_key(color) {
      if let Some(_) = sock_hash.get_mut(color) {
        pair_count += 1;
        sock_hash.remove(color);
      }
    } else {
      sock_hash.insert(color, 1);
    }
  });

  return pair_count;
}

// -------------------------- Find maximum subarray -------------------------

pub fn find_max_subarray_linear_time(arr: &[i32]) -> (i32, usize, usize) {
  let mut max_value = arr[0];
  let mut low = 0;
  let mut high = 1;

  let mut chain_sum = arr[0];
  let mut chain_low = 0;
  let mut chain_high = 0;

  for (i, value) in arr.iter().enumerate() {
    chain_high += 1;
    if i == 0 {
      continue;
    }

    if chain_sum <= 0 {
      chain_sum = *value;
      chain_low = i;
    } else {
      chain_sum += value
    }

    println!("\n{} - {}", i, value);
    println!("chain: {} - {} - {}", chain_low, chain_high, chain_sum);
    println!("max: {} - {} - {}", low, high, max_value);

    if chain_sum >= max_value {
      max_value = chain_sum;
      low = chain_low;
      high = chain_high;
    }
  }

  (max_value, low, high)
}

// --- https://www.hackerrank.com/challenges/jumping-on-the-clouds/problem --

pub fn jumping_on_clouds(arr: &[bool]) -> u16 {
  let mut min_jumps = 0;
  let mut position = 0usize;

  while position != arr.len() - 1 {
    if let Some(true) = arr.get(position + 2) {
      position += 2;
    } else {
      position += 1;
    }

    min_jumps += 1;
  }

  min_jumps
}

// ----- https://www.hackerrank.com/challenges/counting-valleys/problem -----

pub fn counting_valeys(path: &str) -> u32 {
  let mut count = 0u32;
  let mut level = 0;

  for step in path.chars() {
    if step == 'u' {
      level += 1;
      if level == 0 {
        count += 1
      }
    } else {
      level -= 1
    }
  }

  count
}

// ------ https://www.hackerrank.com/challenges/repeated-string/problem -----

pub fn repeated_string(s: &str, n: usize) -> usize {
  if s.len() < 1 {
    return 0;
  }

  let offset = n % s.len();
  let repetition = (n - offset) / s.len();
  let mut count = 0;

  for c in s.chars() {
    if c == 'a' {
      count += 1
    }
  }

  count * repetition + repeated_string(&s[0..offset], s[0..offset].len())
}

// --------- https://www.hackerrank.com/challenges/2d-array/problem ---------

pub fn hourglass_sum(arr: &[Vec<i32>], i: usize, j: usize, width: usize, height: usize) -> isize {
  if width == 1 && height == 1 {
    return find_hourglass_with_center(&arr, i, j);
  }

  let mut _max_1 = isize::MIN;
  let mut _max_2 = isize::MIN;

  if width > height {
    _max_1 = hourglass_sum(&arr, i, j, width / 2, height);
    _max_2 = hourglass_sum(&arr, i, j + width / 2, width / 2, height);
  } else {
    _max_1 = hourglass_sum(&arr, i, j, width, height / 2);
    _max_2 = hourglass_sum(&arr, i + height / 2, j, width, height / 2);
  }

  if _max_1 > _max_2 {
    return _max_1;
  } else {
    return _max_2;
  }
}

pub fn find_hourglass_with_center(arr: &[Vec<i32>], i: usize, j: usize) -> isize {
  return (arr[i - 1][j - 1]
    + arr[i - 1][j]
    + arr[i - 1][j + 1]
    + arr[i][j]
    + arr[i + 1][j - 1]
    + arr[i + 1][j]
    + arr[i + 1][j + 1]) as isize;
}

// ------------- https://leetcode.com/problems/contiguous-array/ ------------

pub fn contiguous_array(arr: &[bool]) -> usize {
  let mut max_length = 0;
  let mut state_to_index: HashMap<i32, usize> = HashMap::new();
  let mut count = 0;

  for (index, value) in arr.iter().enumerate() {
    count += if *value { 1 } else { -1 };

    if let Some(past_index) = state_to_index.get(&count) {
      max_length = std::cmp::max(max_length, index - past_index);
    } else {
      state_to_index.insert(count, index);
    }
  }

  max_length
}

// --------- https://leetcode.com/problems/valid-parenthesis-string/ --------

pub fn valid_parenthesis_string(s: &str) -> bool {
  let mut low = 0;
  let mut high = 0;

  for char in s.chars() {
    match char {
      '(' => {
        low += 1;
        high += 1;
      }
      ')' => {
        low -= 1;
        high -= 1;
      }
      _ => {
        low -= 1;
        high += 1;
      }
    }

    if high < 0 {
      break;
    }

    low = cmp::max(0, low);
  }

  return low == 0;
}

// ------ https://www.hackerrank.com/challenges/minimum-swaps-2/problem -----

pub fn minimum_swap(arr: &mut [i32]) -> u32 {
  let mut count = 0;

  for i in 0..arr.len() {
    let curr_val = arr[i];
    let intended_value = i as i32 + 1;

    if curr_val == intended_value {
      continue;
    }

    for j in i + 1..arr.len() {
      if arr[j] == intended_value {
        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        count += 1;
        break;
      }
    }
  }

  return count;
}

// ----------------- https://leetcode.com/problems/two-sum/ -----------------

pub fn two_sum(nums: &[i32], target: i32) -> Vec<usize> {
  let mut result = vec![];
  let mut map: HashMap<i32, usize> = HashMap::new();

  // let nums = vec![2, 7, 11, 15];
  for (curr_index, num) in nums.iter().enumerate() {
    let complement = target - num;
    if let Some(complement_index) = map.get(&num) {
      result = vec![*complement_index, curr_index];
      break;
    } else {
      map.insert(complement, curr_index);
    }
  }

  result
}

// ----------- https://www.hackerrank.com/challenges/crush/problem ----------

pub fn array_manipulation(length: usize, operations: &[[u32; 3]]) -> u32 {
  let mut max_value = 0;
  let mut arr = vec![0i32; length];

  for operation in operations {
    let start = operation[0] as usize - 1;
    let end = operation[1] as usize;
    let value = operation[2];

    arr[start] += value as i32;
    if end < arr.len() {
      arr[end] -= value as i32;
    }
  }

  arr.iter().fold(0, |peak, value| {
    max_value = std::cmp::max(max_value, peak + value);
    peak + value
  });

  max_value as u32
}

// ------------------ https://leetcode.com/problems/longest-substring-without-repeating-characters/ -----------------

pub fn longest_substring_without_repeating_characters(s: &str) -> usize {
  let mut max_length = 0;
  let mut low = 0;
  let mut dict: HashMap<char, usize> = HashMap::new();

  for (high, char) in s.char_indices() {
    if let Some(old_index) = dict.insert(char, high) {
      low = cmp::max(low, old_index + 1);
    }
    max_length = cmp::max(max_length, high - low + 1);
  }

  max_length
}

// ---------------------------------------------- Rod cutting problem. ----------------------------------------------

// Given a steel rod and a price-for-inch table.
// Find optimal cutting strategy for the most value (No cut maybe needed)
pub fn rod_cutting(length: usize, price_table: &[usize], cut_price: usize) -> (usize, String) {
  let mut max_value_for_length = vec![0];
  let mut optimal_cut_for_length = vec![0];

  let mut max_value = 0;
  let mut cuts = String::new();

  for i in 1..=length {
    max_value_for_length.push(0);
    optimal_cut_for_length.push(0);

    for j in 0..i {
      let value = (max_value_for_length[j] + price_table[i - j]).saturating_sub(if j > 1 {
        cut_price
      } else {
        0
      });
      if value > max_value_for_length[i] {
        max_value_for_length[i] = value;
        optimal_cut_for_length[i] = j;
      }
    }

    max_value = cmp::max(max_value, max_value_for_length[i]);
  }

  let mut curr_length = length;
  loop {
    let cut = if optimal_cut_for_length[curr_length] == 0 {
      curr_length
    } else {
      optimal_cut_for_length[curr_length]
    };
    cuts.push_str(&cut.to_string());
    curr_length -= cut;
    if curr_length == 0 {
      break;
    } else {
      cuts.push('+');
    }
  }

  (max_value, cuts)
}

// ----------------------------------------------- FIBONACCI GENERATOR ----------------------------------------------

pub fn fibonacci_generator(n: usize) -> usize {
  let mut prev = 0;
  let mut curr = 1;

  for _ in 3..=n {
    let temp = curr;
    curr += prev;
    prev = temp;
  }

  match n {
    1 => 0,
    2 => 1,
    _ => curr,
  }
}

// --------------------------- https://leetcode.com/problems/median-of-two-sorted-arrays/ ---------------------------

pub fn median_of_two_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> f32 {
  let mut index_1 = 0;
  let mut index_2 = 0;
  let mut arr3 = vec![];

  loop {
    let value_1 = if index_1 < arr1.len() {
      arr1[index_1]
    } else {
      i32::MAX
    };

    let value_2 = if index_2 < arr2.len() {
      arr2[index_2]
    } else {
      i32::MAX
    };

    if index_1 >= arr1.len() && index_2 >= arr2.len() {
      break;
    }

    if value_1 <= value_2 {
      arr3.push(value_1);
      index_1 += 1;
    } else {
      arr3.push(value_2);
      index_2 += 1;
    }
  }

  let mid = arr3.len() / 2;
  if arr3.len() % 2 == 0 {
    (arr3[mid] + arr3[mid - 1]) as f32 / 2f32
  } else {
    arr3[mid] as f32
  }
}

// -------------------------- https://leetcode.com/problems/longest-palindromic-substring/ --------------------------

pub fn longest_palindromic_sub_string(s: &str) -> usize {
  let chars: Vec<char> = s.chars().collect();
  let mut max_palindrom_length = 0;
  let mut char_to_pos: HashMap<&char, Vec<usize>> = HashMap::new();

  for (curr_pos, char) in chars.iter().enumerate() {
    if let Some(char_pos) = char_to_pos.get_mut(&char) {
      for j in 0..char_pos.len() {
        let sub_str = &s[char_pos[j]..curr_pos + 1];

        if sub_str.len() > max_palindrom_length {
          if check_is_palindrom(&sub_str.chars().collect::<Vec<char>>()) {
            max_palindrom_length = sub_str.len();
          } else {
            continue;
          }
        }

        break;
      }

      char_pos.push(curr_pos);
    } else {
      max_palindrom_length = cmp::max(max_palindrom_length, 1);
      char_to_pos.insert(&char, vec![curr_pos]);
    }
  }

  max_palindrom_length
}

fn check_is_palindrom(chars: &[char]) -> bool {
  let mid = chars.len() / 2;
  let mut left_index = mid - 1;
  let mut right_index = if chars.len() % 2 == 0 { mid } else { mid + 1 };

  loop {
    if chars[left_index] != chars[right_index] {
      break false;
    }
    if left_index == 0 && right_index == chars.len() - 1 {
      break true;
    } else {
      left_index -= 1;
      right_index += 1;
    }
  }
}

// -------------------------------- https://leetcode.com/problems/zigzag-conversion/ --------------------------------

pub fn zigzag_conversion(s: &str, num_rows: usize) -> String {
  let chars: Vec<char> = s.chars().collect();
  let offset = 2 * num_rows - 2;
  let mut new_s = String::new();

  for i in 1..=num_rows {
    let mut j = i - 1;
    while j < chars.len() {
      new_s.push(chars[j]);

      let inner_offset = 2 * (num_rows - i);
      if i > 1 && i < num_rows && (j + inner_offset < chars.len()) {
        new_s.push(chars[j + inner_offset]);
      }

      j += offset;
    }
  }

  new_s
}

// --------------------------------- https://leetcode.com/problems/reverse-integer/ ---------------------------------

pub fn reverse_integer(x: i32) -> i32 {
  let reverse_digits = String::from_iter(x.to_string().chars().rev());

  if let Ok(num) = reverse_digits.parse::<i32>() {
    num
  } else {
    0
  }
}

// ------------------------------ https://leetcode.com/problems/string-to-integer-atoi/ -----------------------------

pub fn string_to_integer(s: &str) -> i32 {
  let s = trim_leading_whitespace(s);
  let non_negative = determine_sign(&s);

  let mut converted_num = 0i32;
  for (index, char) in s.char_indices() {
    if let Some(digit) = char.to_digit(10) {
      converted_num = converted_num
        .saturating_mul(10)
        .saturating_sub(digit as i32);
    } else if index == 0 && (char == '+' || char == '-') {
      continue;
    } else {
      break;
    }
  }

  if non_negative {
    converted_num.saturating_neg()
  } else {
    converted_num
  }
}

fn trim_leading_whitespace(s: &str) -> String {
  let mut iter = s.char_indices().filter(|char| char.1 != ' ');
  if let Some((index, _)) = iter.next() {
    String::from(&s[index..])
  } else {
    String::new()
  }
}

fn determine_sign(s: &str) -> bool {
  if &s[0..1] == "-" {
    false
  } else {
    true
  }
}

// --------------------------- https://leetcode.com/problems/regular-expression-matching/ ---------------------------

pub fn regular_expression_matching(s: &str, p: &str) -> bool {
  let mut results = vec![vec![false; p.len() + 1]; s.len() + 1];
  results[s.len()][p.len()] = true;

  for s_index in (0..=s.len()).rev() {
    for (p_index, p_char) in p.char_indices().rev() {
      let first_match = s_index < s.len()
        && (s.get(s_index..s_index + 1) == p.get(p_index..p_index + 1) || p_char == '.');

      if let Some("*") = p.get(p_index + 1..p_index + 2) {
        let case_1 = if let Some(row) = results.get(s_index + 1) {
          row[p_index]
        } else {
          true
        };

        let case_2 = *results[s_index].get(p_index + 2).unwrap_or(&false);

        results[s_index][p_index] = (first_match && case_1) || case_2
      } else {
        results[s_index][p_index] = first_match && results[s_index + 1][p_index + 1]
      }
    }
  }

  results[0][0]
}

// --------------------------------- https://leetcode.com/problems/integer-to-roman/ --------------------------------

pub fn integer_to_roman(num: u32) -> String {
  let roman_digits = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
  let mut num = num;
  let mut roman_num = String::new();

  for i in (0..=3).rev() {
    let modifier = 10u32.pow(i as u32);
    let digit = num / modifier;

    if digit < 4 {
      for _ in 0..digit {
        roman_num.push(roman_digits[2 * i]);
      }
    } else if digit < 5 {
      roman_num.push(roman_digits[2 * i]);
      roman_num.push(roman_digits[2 * i + 1]);
    } else if digit < 9 {
      roman_num.push(roman_digits[2 * i + 1]);
      for _ in 0..(digit - 5) {
        roman_num.push(roman_digits[2 * i]);
      }
    } else {
      roman_num.push(roman_digits[2 * i]);
      roman_num.push(roman_digits[2 * i + 2]);
    }

    num %= modifier;
  }

  roman_num
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn sock_merchant_test() {
    let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    let pair_count = sock_merchant(&ar);
    let expected = 3u32;

    assert_eq!(pair_count, expected);
  }

  #[test]
  fn jumping_on_cloud_test() {
    let arr = vec![true, true, true, true, false, true];

    let min_jumps = jumping_on_clouds(&arr);
    let expected = 3u16;

    assert_eq!(min_jumps, expected);
  }

  #[test]
  fn counting_valley_test() {
    let path = "uddduduu";

    let count = counting_valeys(&path);
    let expected = 1u32;

    assert_eq!(count, expected)
  }

  #[test]
  fn repeated_string_test() {
    let s = "epsxyyflvrrrxzvnoenvpegvuonodjoxfwdmcvwctmekpsnamchznsoxaklzjgrqruyzavshfbmuhdwwmpbkwcuomqhiyvuztwvq";
    let n = 549382313570;

    let count = repeated_string(&s, n);
    let expected = 16481469408;

    assert_eq!(count, expected);
  }

  #[test]
  fn find_max_subarray_test() {
    let arr = vec![
      13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];

    let max_subarray = find_max_subarray_linear_time(&arr);
    let expected = (43, 7, 11);

    assert_eq!(max_subarray, expected);
  }

  #[test]
  fn hourglass_sum_test() {
    let arr = [
      vec![-9, -9, -9, 1, 1, 1],
      vec![0, -9, 0, 4, 3, 2],
      vec![-9, -9, -9, 1, 2, 3],
      vec![0, 0, 8, 6, 6, 0],
      vec![0, 0, 0, -2, 0, 0],
      vec![0, 0, 1, 2, 4, 0],
    ];

    let result = hourglass_sum(&arr, 1, 1, 4, 4);
    let expected = 28;

    assert_eq!(result, expected)
  }

  #[test]
  fn contiguous_array_test() {
    let arr = vec![true, true, true, false, false];

    let result = contiguous_array(&arr);
    let expected = 4;

    assert_eq!(result, expected);
  }

  #[test]
  fn valid_parenthesis_string_test() {
    let s = "(*)";

    let result = valid_parenthesis_string(s);
    let expected = true;

    assert_eq!(result, expected)
  }

  #[test]
  fn tow_sum_test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum(&nums, target);
    let expected = vec![0, 1];

    assert_eq!(result, expected);
  }

  #[test]
  fn array_manipulation_test() {
    let n = 5;
    let operations = vec![[1, 2, 100], [2, 5, 100], [3, 4, 100]];

    let result = array_manipulation(n, &operations);
    let expected = 200;

    assert_eq!(result, expected);
  }

  #[test]
  fn longest_substring_without_repeating_characters_test() {
    let s1 = "abcabcbb";
    let s2 = "bbbb";
    let s3 = "pwwkew";
    let s4 = "wobgrovw";
    let s5 = " ";

    assert_eq!(longest_substring_without_repeating_characters(&s1), 3);
    assert_eq!(longest_substring_without_repeating_characters(&s2), 1);
    assert_eq!(longest_substring_without_repeating_characters(&s3), 3);
    assert_eq!(longest_substring_without_repeating_characters(&s4), 6);
    assert_eq!(longest_substring_without_repeating_characters(&s5), 1);
  }

  #[test]
  fn rod_cutting_test() {
    let price_table = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
    let length = 4;
    let cut_price = 1;

    let result = rod_cutting(length, &price_table, cut_price);
    let expected = (9, String::from("4"));

    assert_eq!(result, expected);
  }

  #[test]
  fn fibonacci_generator_test() {
    let n = 50;

    let result = fibonacci_generator(n);
    let expected = 7778742049;

    assert_eq!(result, expected)
  }

  #[test]
  fn median_of_two_sorted_arrays_test() {
    let arr1 = [2];
    let arr2 = [];

    let result = median_of_two_sorted_arrays(&arr1, &arr2);
    let expected = 2f32;

    assert_eq!(result, expected);
  }

  #[test]
  fn longest_palindromic_sub_string_test() {
    let s = "ac";

    let result = longest_palindromic_sub_string(s);
    let expected = 1;

    assert_eq!(result, expected);
  }

  #[test]
  fn zigzag_conversion_test() {
    let s = "PAYPALISHIRING";

    let result = zigzag_conversion(s, 4);
    let expected = String::from("PINALSIGYAHRPI");

    assert_eq!(result, expected);
  }

  #[test]
  fn reverse_interger_test() {
    let x = 2i32.pow(30) / 10;

    let result = reverse_integer(x);
    let expected = 281473701;

    assert_eq!(result, expected)
  }

  #[test]
  fn string_to_integer_test() {
    let s = "words and 987";

    let result = string_to_integer(s);
    let expected = 0;

    assert_eq!(result, expected)
  }

  #[test]
  fn integer_to_roman_test() {
    let num = 9;

    let result = integer_to_roman(num);
    let expected = String::from("IX");

    assert_eq!(result, expected)
  }
}
