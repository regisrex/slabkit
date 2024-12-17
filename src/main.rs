use evaluator::Evaluator;
use parser::Parser;
use clap::Parser as ClapParser;

mod file_ops;
mod logger;
mod utils;
mod lexer;
mod parser;
mod  html;
mod evaluator;

#[derive(ClapParser, Debug)]
#[command(author = "Regis Rex https://github.com/regisrex", version = "0.0.1", about ="SlabKit, an html templating engine that is easy to use", long_about = None)]
pub struct Args {
    /// Action to perform on the template  ['dev', 'compile']
    #[arg(short, long)]
    action: Option<String>,

    /// Path to the template file
    #[arg(short, long)]
    template: Option<String>,

    /// Path to the output file
    #[arg(short, long)]
    output: Option<String>,

    /// Path to the data file
    /// This is the data that will be used to populate the template
    #[arg(short, long)]
    data: Option<String>,
}

fn main() {
    let args = Args::parse();

    let action = utils::get_action(&args.action);

    let template = utils::get_template(&args.template);
    // println!("Template: {}", template);

    let data = utils::get_data(&args.data);

    let output = utils::get_outfile(&args.output);

    let parser_output_node = Parser::new(template).parse();
    match parser_output_node {
        Ok(node) => {
            let json_value = file_ops::get_json_value_from_template(data.unwrap() );
            let evaluated_node = Evaluator::new(json_value).evaluate(node);
        },
        Err(error) => {
            panic!("Slabkit error: {:?}", error)
        }
    }

    // let final_data = compile_slk_contents(&template, &data);
    // println!("Final data: {}", final_data);
}
