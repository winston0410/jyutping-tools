pub struct Segmentator {}

impl Segmentator {
    fn greet(self) -> () {
        println!("hello");
    }
}

#[test]
fn test_greet() {
    let segmentator = Segmentator {};
    assert_eq!(segmentator.greet(), ());
}
