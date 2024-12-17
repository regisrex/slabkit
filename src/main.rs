use clap::Parser as ClapParser;
use evaluator::Evaluator;
use parser::Parser;
use warp::Filter;

mod evaluator;
mod file_ops;
mod html;
mod lexer;
mod logger;
mod parser;
mod utils;

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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let action = utils::get_action(&args.action);

    let template = utils::get_template(&args.template);
    // println!("Template: {}", template);

    let data = utils::get_data(&args.data);

    let output = utils::get_outfile(&args.output);

    let parser_output_node = Parser::new(template).parse();

    match parser_output_node {
        Ok(node) => {
            let json_value = file_ops::get_json_value_from_template(data.unwrap());
            let evaluated_node = Evaluator::new(json_value).evaluate(node);

            // Define a warp filter that serves the HTML
            let html_route = warp::path::end().map(move || warp::reply::html(evaluated_node.node_to_html().clone()));

            // Start the warp server
            println!("Server running at http://localhost:3030");
            warp::serve(html_route).run(([127, 0, 0, 1], 3030)).await;
        }
        Err(error) => {
            panic!("Slabkit error: {:?}", error)
        }
    }
}
