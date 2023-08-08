use super::Solution;

impl Solution {
    /// Solution for [Leetcode Problem #13: Roman to Integer](https://leetcode.com/problems/roman-to-integer/)
    ///
    /// Given a roman numeral, converts it to an integer.
    ///
    /// # Example
    /// ```rust
    /// use algo::leetcode::Solution;
    ///
    /// let three = Solution::roman_to_int(format!("III"));
    /// assert_eq!(three, 3);
    /// ```
    ///
    /// # Problem Solving
    /// The first problem is the ability for smaller digits to subtract from
    /// larger ones.
    ///
    /// For example, `"IV" == 4` because the V digit is preceded by an I digit,
    /// and subtracts 1 from 5.
    ///
    /// First off though, we need a function to translate from single roman
    /// numerals to integers. (Done through `Self::single_digit_roman_to_int()`)
    ///
    /// The big idea is we'll iterate over each character of the string. We look
    /// at the 'primary' index, and the next index. If the next index does not
    /// exist, or is not 'subtractable', we add the numeral to the accumulator.
    /// If the next index is subtractable, we add the subtracted amount to the
    /// accumulator, and skip over to the i+2 index.
    pub fn roman_to_int(s: String) -> i32 {
        let mut i = 0;
        let mut acc = 0;
        while i < s.chars().count() {
            match (s.chars().nth(i), s.chars().nth(i + 1)) {
                (None, _) => panic!("Index out of range"),
                (Some(char), None) => {
                    acc += Self::single_digit_roman_to_int(char) as i32;
                    i += 1
                }
                (Some(primary_char), Some(next_char)) => {
                    if Self::check_subtraction(primary_char, next_char) == true {
                        acc += (Self::single_digit_roman_to_int(next_char)
                            - Self::single_digit_roman_to_int(primary_char))
                            as i32;
                        i += 2;
                    } else {
                        acc += Self::single_digit_roman_to_int(primary_char) as i32;
                        i += 1
                    }
                }
            }
        }
        return acc;
    }

    fn single_digit_roman_to_int(c: char) -> u16 {
        return match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => {
                panic!("Not a valid character!")
            }
        };
    }

    fn check_subtraction(primary_char: char, next_char: char) -> bool {
        return match primary_char {
            'I' => match next_char {
                'V' => true,
                'X' => true,
                _ => false,
            },
            'X' => match next_char {
                'L' => true,
                'C' => true,
                _ => false,
            },
            'C' => match next_char {
                'D' => true,
                'M' => true,
                _ => false,
            },
            _ => false,
        };
    }
}

#[test]
fn example_1() {
    assert_eq!(Solution::roman_to_int(format!("III")), 3);
}
#[test]
fn example_2() {
    assert_eq!(Solution::roman_to_int(format!("LVIII")), 58);
}
#[test]
fn example_3() {
    assert_eq!(Solution::roman_to_int(format!("MCMXCIV")), 1994);
}
