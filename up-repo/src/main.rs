mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod git;

// Main Entry Point
fn main() -> () {
    tokenize_arguments();

    return ();
}
