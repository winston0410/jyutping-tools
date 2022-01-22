use rscantonese;

#[test]
fn test_add() {
    assert_eq!(rscantonese::sum(4, 5).unwrap(), 9);
}
