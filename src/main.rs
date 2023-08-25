use std::process::exit;

mod query;

fn main() {
    let input = match query::InputParam::try_from(std::env::args()) {
        Ok(params) => params,
        Err(e) => {println!("Error: {}", e); usage(); exit(1)}
    };

    let result = query::query(&input).unwrap();
    println!("{}", result);
}

fn usage() {
    println!("hq [--url|-u URL] <--xpath XPATH | --css CSS-SELECTOR>\n
             If no --url or -u specified, read from stdin.
             --xpath | -x XPATH. xpath to select the document node. By default it output the text of the selected nodes; Use '@' to select the property. e.g.: //div@name
             --css | -c SELECTOR select the document node. By default it output the text of the selected nodes; Use '@' to select the property. e.g.: div@name");
}

