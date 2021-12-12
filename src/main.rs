use rand::seq::SliceRandom;

mod nouns;
use nouns::NOUNS;

mod verbs;
use verbs::VERBS;

mod adjectives;
use adjectives::ADJECTIVES;

fn main() {
    let first_adjective: &str = ADJECTIVES.choose(&mut rand::thread_rng()).unwrap();
    let first_noun: &str = NOUNS.choose(&mut rand::thread_rng()).unwrap();
    let first_verb: &str = VERBS.choose(&mut rand::thread_rng()).unwrap();
    let second_noun: &str = NOUNS.choose(&mut rand::thread_rng()).unwrap();

    // BUG: I don't know how to fix this compiler warning for line 25:
    //
    // warning: value assigned to `indefinite_article` is never read
    // note: `#[warn(unused_assignments)]` on by default
    // help: maybe it is overwritten before being read?
    //
    // If you know how to fix this, please create a branch on the GitHub repo with the fix.
    let mut indefinite_article: &str = "";

    if second_noun.split_at(1).0 == "a"
        || second_noun.split_at(1).0 == "e"
        || second_noun.split_at(1).0 == "i"
        || second_noun.split_at(1).0 == "o"
        || second_noun.split_at(1).0 == "u"
    {
        indefinite_article = "an";
    } else {
        indefinite_article = "a";
    }

    println!(
        "The {} {} {} {} {}.",
        first_adjective, first_noun, first_verb, indefinite_article, second_noun
    );
}
