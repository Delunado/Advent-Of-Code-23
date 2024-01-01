pub fn contains_digit_as_string(string_to_check: &str) -> Option<char> {
    if string_to_check.contains("one") {
        return Some('1');
    }

    if string_to_check.contains("two") {
        return Some('2');
    }

    if string_to_check.contains("three") {
        return Some('3');
    }

    if string_to_check.contains("four") {
        return Some('4');
    }

    if string_to_check.contains("five") {
        return Some('5');
    }

    if string_to_check.contains("six") {
        return Some('6');
    }

    if string_to_check.contains("seven") {
        return Some('7');
    }

    if string_to_check.contains("eight") {
        return Some('8');
    }

    if string_to_check.contains("nine") {
        return Some('9');
    }

    return None;
}
