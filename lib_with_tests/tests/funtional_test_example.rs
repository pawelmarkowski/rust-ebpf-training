// I know it is not a real world example, but it is a good way to
// demonstrate how to write tests.

use lib_with_tests::example_arrays;

#[test]
fn test_example_arrays() {
    assert_eq!(example_arrays(), [1, 2, 3, 4, 5]);
    assert_eq!(example_arrays().len(), 5);
    assert_eq!(example_arrays()[0], 1);
}
