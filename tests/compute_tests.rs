use rtropy;

fn float_eq(fst: rtropy::Float, snd: rtropy::Float) -> bool {
    return (fst - snd).abs() < rtropy::Float::EPSILON;
}

#[test]
fn compute_test() {
    let data: Vec<(&str, rtropy::Float)> = vec![
        ("", 0.0),
        ("aaa", 0.0),
        ("aaabbb", 1.0),
        ("ppplllppplll", 1.0),
        ("abcabcabc", 1.585),
        ("abcd", 2.0),
        ("abcdefgz", 3.0),
        ("qertyuiopasdfghk", 4.0),
        ("predicate", 2.9477),
        ("import from file", 3.25),
        ("haskell one love", 3.1556),
        ("entropy calculator", 3.5033),
        ("C++", 0.9183),
    ];

    for i in data {
        let str = i.0;
        let counts = rtropy::count(str.chars());
        let numbers = counts.iter().map(|i| i.1);
        let actual = rtropy::compute(numbers.collect());
        let actual = rtropy::fround(actual, 4);

        let expected = i.1;

        assert!(float_eq(actual, expected), "test for {str}");
    }
}
