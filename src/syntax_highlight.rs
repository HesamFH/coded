use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref definitions: HashSet<&'static str> = HashSet::from([
        "def",
        "let",
        "var",
        "const",
        "fn",
        "func",
        "function",
        "return",
        "import",
        "include",
        "use",
        "async",
        "await",
        "pub",
        "public",
        "private",
        "protected",
        "as",
    ]);
    static ref conditions_and_loops: HashSet<&'static str> = HashSet::from([
        "if", "else", "elif", "for", "while", "do", "loop","then", "fi", "==", "!=", ">", "<", ">=", "<="
    ]);
    static ref datatypes: HashSet<&'static str> = HashSet::from([
        "int", "float", "string", "str", "bool", "boolean", "uint", "void", "mod", "i8", "i16",
        "i32", "i64", "u8", "u16", "u32", "u64", "f8", "f16", "f32", "f64"
    ]);
}

pub enum WordType {
    Definition,
    Condition,
    Datatype,
    String,
    Default,
}

pub fn get_word_type(word: &str, is_string: &bool) -> WordType {
    if *is_string {
        WordType::String
    } else {
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
}
