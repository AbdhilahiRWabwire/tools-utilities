mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod upgrades;

// Main Entry Point
fn main() -> () {
    tokenize_arguments();

    return ();
}
