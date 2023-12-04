#[test]
fn test_line_parsing(){
    let cases = [
        ("1abc2", 12),
        ("treb7uchet", 77),
        ("4twoscpht", 44)
    ];

    for (line, expected) in cases.iter(){
        let actual = super::calc_calibration_value(line, false);
        assert_eq!(*expected, actual);
    }
}

#[test]
fn test_calc_calibration_values(){
    let expected = 142;

    let actual = super::calc_calibration_values("resources/2023/day01/test_input.txt", false);

    assert_eq!(expected, actual);
}

#[test]
fn test_calc_calibration_values_with_words(){
    let expected = 281;

    let actual = super::calc_calibration_values("resources/2023/day01/test_input2.txt", true);

    assert_eq!(expected, actual);
}