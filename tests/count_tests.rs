use rtropy;

#[test]
fn count_test() {
    let data = vec![
        ("", vec![]),
        ("aaa", vec![('a', 3)]),
        ("aaabbb", vec![('a', 3), ('b', 3)]),
        ("ppplllppplll", vec![('l', 6), ('p', 6)]),
        ("abcabcabc", vec![('a', 3), ('b', 3), ('c', 3)]),
    ];

    for i in data {
        let str = i.0;
        let mut actual = rtropy::count(String::from(str).chars());
        actual.sort_by_key(|i| i.0);

        let expected = i.1;

        assert_eq!(actual, expected, "test for {str}")
    }
}
