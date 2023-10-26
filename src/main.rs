use std::{fs::{File, self}, error::Error, process::ExitCode};
use std::io::Write;

use biblatex::Bibliography;

use indoc::indoc;

fn main() -> Result<ExitCode, Box<dyn Error>>{
    let entry = "
        @inproceedings{DBLP:conf/fscd/ErkensL20,
            author       = {Rick Erkens and
                            Maurice Laveaux},
            editor       = {Zena M. Ariola},
            title        = {Adaptive Non-Linear Pattern Matching Automata},
            booktitle    = {5th International Conference on Formal Structures for Computation
                            and Deduction, {FSCD} 2020, June 29-July 6, 2020, Paris, France (Virtual
                            Conference)},
            series       = {LIPIcs},
            volume       = {167},
            pages        = {20:1--20:21},
            publisher    = {Schloss Dagstuhl - Leibniz-Zentrum f{\"{u}}r Informatik},
            year         = {2020},
            url          = {https://doi.org/10.4230/LIPIcs.FSCD.2020.20},
            doi          = {10.4230/LIPICS.FSCD.2020.20},
            timestamp    = {Tue, 30 Jun 2020 11:42:59 +0200},
            biburl       = {https://dblp.org/rec/conf/fscd/ErkensL20.bib},
            bibsource    = {dblp computer science bibliography, https://dblp.org}
        }
        ";

    let result = Bibliography::parse(entry);

    println!("{:?}", result);

    // Create the output directory
    fs::create_dir_all("tmp/")?;

    // Write the output file.
    let output = File::create("tmp/index.html")?;
    writeln!(&output, indoc!{"
    <!doctype html>
    <html>
      <head>
        <title>This is the title of the webpage!</title>
      </head>
      <body>
        <p>This is an example paragraph. Anything in the <strong>body</strong> tag will appear on the page, just like this <strong>p</strong> tag and its contents.</p>
      </body>
    </html>
    "})?;

    Ok(ExitCode::SUCCESS)
}
