use schindel::shingles::{MinShingleHash, Murmur3Hasher};

fn main() {
    let original = "\
        “My sight is failing,” she said finally. “Even when I was young I could not have read what was written there. \
        But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be, \
        Benjamin?” For once Benjamin consented to break his rule, and he read out to her what was written on the wall. \
        There was nothing there now except a single Commandment. It ran:\
        ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";

    let plagiarism = "\
        “My sight is failing,” she said finally. “When I was young I could not have read what was written there. \
        But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be” \
        Benjamin read out to her what was written. There was nothing there now except a single Commandment. \
        It ran: ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";

    let other = "\
        Throughout the spring and summer they worked a sixty-hour week, and in August Napoleon announced that there \
        would be work on Sunday afternoons as well. This work was strictly voluntary, but any animal who absented \
        himself from it would have his rations reduced by half. Even so, it was found necessary to leave certain \
        tasks undone. The harvest was a little less successful than in the previous year, and two fields which \
        should have been sown with roots in the early summer were not sown because the ploughing had not been \
        completed early enough. It was possible to foresee that the coming winter would be a hard one.";

    const HASH_LEN: usize = 100;
    const NGRAM_LEN: usize = 5;

    let original_hash = MinShingleHash::<Murmur3Hasher, HASH_LEN, NGRAM_LEN>::new(original.chars());

    let plagiarism_hash = MinShingleHash::<Murmur3Hasher, HASH_LEN, NGRAM_LEN>::new(plagiarism.chars());
    println!("plagiarism similarity: {}", original_hash.compare(&plagiarism_hash));

    let other_hash = MinShingleHash::<Murmur3Hasher, HASH_LEN, NGRAM_LEN>::new(other.chars());
    println!("other text similarity: {}", original_hash.compare(&other_hash));
}
