use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
        let mut output = String::new();
        for token in input.graphemes(true) {
            output.insert_str(0, token);
        }
        output
}

#[test]
fn grapheme_cluster_with_pre_combined_form() {
    let input = "Würstchenstand";
    let output = reverse(input);
    let expected = "dnatsnehctsrüW";
    assert_eq!(output, expected);
}