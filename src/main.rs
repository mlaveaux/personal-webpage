use std::{fs::{File, self}, error::Error, process::ExitCode};
use std::io::Write;

use biblatex::Bibliography;
use markdown::to_html;
use itertools::Itertools;

use indoc::indoc;

fn main() -> Result<ExitCode, Box<dyn Error>>{

    // Read the bibliography to produce a nice table
    // TODO: Handle error.
    let bibliography = Bibliography::parse(include_str!("../data/bibliography.bib")).unwrap();

    let markdown_str = format!(
      indoc! {
        "# Publications
        {}"
      },
      bibliography
        .iter()
        .map(|entry| {
          format!("{}
            __{}__: {}", 
            entry.fields["author"][0].v.get(),
            entry.fields["title"][0].v.get(),
            entry.fields["booktitle"][0].v.get()
          )
        }).join(" * ")
    );

    // Create the output directory
    fs::create_dir_all("tmp/")?;

    // Write the HTML itself.
    let output = File::create("tmp/index.html")?;
    writeln!(&output, indoc!{"
      <!doctype html>
      <html>
        <head>
          <title>My personal webpage!</title>
        </head>
        <body>
          {}
        </body>
      </html>"}, 
      to_html(&markdown_str))?;

    Ok(ExitCode::SUCCESS)
}
