mod parse;

use parse::analyze_text;

fn main() {
    let raw_sample = r#"
    The rain came down hard on 4th Street. I pulled my fedora down low and stepped inside.
    "You late," the barkeep grunted, wiping a spot on the mahogany counter.
    "Traffic was dead," I replied. "Give me a rye, neat."
    "#;

    println!("=== Processing Noir Sample ===");
    let metrics = analyze_text(raw_sample);
    println!("{:#?}", metrics);
}
