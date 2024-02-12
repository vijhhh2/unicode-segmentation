use node_bindgen::derive::node_bindgen;
use tslink::tslink;
use unicode_segmentation::UnicodeSegmentation;

#[tslink(snake_case_naming)]
#[node_bindgen]
fn graphemes(string_to_split: String) -> Vec<String> {
    let mut grapheme_clusters = vec![];
    let graphemes =
        UnicodeSegmentation::graphemes(string_to_split.as_str(), true).collect::<Vec<&str>>();
    for grapheme in graphemes {
        grapheme_clusters.push(grapheme.to_string())
    }
    grapheme_clusters
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn grapheme_indices(string_to_split: String) -> Vec<(i32, String)> {
    let mut grapheme_indices_to_return = vec![];
    let grapheme_indices = UnicodeSegmentation::grapheme_indices(string_to_split.as_str(), true)
        .collect::<Vec<(usize, &str)>>();

    for (index, grapheme) in grapheme_indices {
        grapheme_indices_to_return.push((index as i32, grapheme.to_string()));
    }

    grapheme_indices_to_return
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn unicode_words(string_to_split: String) -> Vec<String> {
    let mut unicode_words_to_return = vec![];
    let unicode_words = UnicodeSegmentation::unicode_words(string_to_split.as_str())
        .collect::<Vec<&str>>();

    for unicode_word in unicode_words {
        unicode_words_to_return.push(unicode_word.to_string());
    }

    unicode_words_to_return
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn unicode_words_indices(string_to_split: String) -> Vec<(i32, String)> {
    let mut unicode_word_indices_to_return = vec![];
    let unicode_word_indices = UnicodeSegmentation::unicode_word_indices(string_to_split.as_str())
        .collect::<Vec<(usize, &str)>>();

    for (index, unicode_word) in unicode_word_indices {
        unicode_word_indices_to_return.push((index as i32, unicode_word.to_string()));
    }

    unicode_word_indices_to_return
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn split_word_bounds(string_to_split: String) -> Vec<String> {
    let mut word_bounds_to_return = vec![];
    let word_bounds = UnicodeSegmentation::split_word_bounds(string_to_split.as_str())
        .collect::<Vec<&str>>();

    for word_bound in word_bounds {
        word_bounds_to_return.push(word_bound.to_string());
    }

    word_bounds_to_return
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn split_word_bound_indices(string_to_split: String) -> Vec<(i32, String)> {
    let mut word_bound_indices_to_return = vec![];
    let word_bound_indices = UnicodeSegmentation::split_word_bound_indices(string_to_split.as_str())
        .collect::<Vec<(usize, &str)>>();

    for (index, word_bound) in word_bound_indices {
        word_bound_indices_to_return.push((index as i32, word_bound.to_string()));
    }

    word_bound_indices_to_return
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn unicode_sentences(string_to_split: String) -> Vec<String> {
    let mut unicode_sentences_to_return = vec![];
    let unicode_sentences = UnicodeSegmentation::unicode_sentences(string_to_split.as_str())
        .collect::<Vec<&str>>();

    for unicode_sentence in unicode_sentences {
        unicode_sentences_to_return.push(unicode_sentence.to_string());
    }

    unicode_sentences_to_return
}


#[tslink(snake_case_naming)]
#[node_bindgen]
fn split_sentence_bounds(string_to_split: String) -> Vec<String> {
    let mut sentence_bounds_to_return = vec![];
    let sentence_bounds = UnicodeSegmentation::split_sentence_bounds(string_to_split.as_str())
        .collect::<Vec<&str>>();

    for sentence_bound in sentence_bounds {
        sentence_bounds_to_return.push(sentence_bound.to_string());
    }

    sentence_bounds_to_return
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn split_sentence_bound_indices(string_to_split: String) -> Vec<(i32, String)> {
    let mut sentence_bound_indices_to_return = vec![];
    let sentence_bound_indices = UnicodeSegmentation::split_sentence_bound_indices(string_to_split.as_str())
        .collect::<Vec<(usize, &str)>>();

    for (index, sentence_bound) in sentence_bound_indices {
        sentence_bound_indices_to_return.push((index as i32, sentence_bound.to_string()));
    }

    sentence_bound_indices_to_return
}
