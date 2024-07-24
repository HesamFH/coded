use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
static ref definitions: HashSet<&'static str> =
    HashSet::from(["def", "let", "var", "const", "fn", "func", "function", "return", "import", "include", "use"]);
static ref conditions_and_loops: HashSet<&'static str> =
    HashSet::from(["if", "else", "elif", "for", "while", "do", "loop", "==", "!=", ">", "<", ">=", "<="]);
static ref datatypes: HashSet<&'static str> = HashSet::from([
    "int", "float", "string", "str", "bool", "boolean", "uint", "void",
]);
}

pub enum WordType {
    Definition,
    Condition,
    Datatype,
    Default,
}

pub fn get_word_type(word: &str) -> WordType {
    let word_trimmed: &str = &word.trim().to_lowercase();
    if definitions.contains(&word_trimmed) {
        WordType::Definition
    } else if conditions_and_loops.contains(&word_trimmed) {
        WordType::Condition
    } else if datatypes.contains(&word_trimmed) {
        WordType::Datatype
    } else {
        WordType::Default
    }
}
