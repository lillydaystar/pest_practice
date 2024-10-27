use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let successful_parse1 = Grammar::parse(Rule::field, "-273.15")?;
    println!("{:?}", successful_parse1);

    let successful_parse2 = Grammar::parse(Rule::record, "-14.7,19.2,-0.75")?;
    println!("{:?}", successful_parse2);

    let unparsed_file = fs::read_to_string("files/numbers.csv").expect("cannot read file");

    let successful_parse3 = Grammar::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap();
    println!("{:?}", successful_parse3);

    Ok(())
}
