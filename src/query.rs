mod css_selector;
mod xpath;

use std::{io::Result, io::Error, io::ErrorKind, default, collections::HashMap, env::Args};

pub fn query(input: &InputParam) -> Result<String> {
    let doc = read_doc(&input)?;
    let selected = match &input.node_selector {
        NodeSelector::Css(ref select) => {css_selector::query(&select, doc)},
        NodeSelector::Xpath(ref select) => {xpath::query(&select, doc)}
    }?;
    Ok(selected.join("\n"))
}

fn read_doc(input: &InputParam) -> Result<String> {
    match &input.data_source {
        DataSource::URL(_url) => Err(Error::new(ErrorKind::Unsupported, "Unsupported yet")),
        DataSource::TEXT(text) => Ok(text.clone())
    }
}

enum NodeSelector {
    Css(String),
    Xpath(String)
}
/// Input Parameters 
pub struct InputParam {
    data_source: DataSource,
    node_selector: NodeSelector
}

impl InputParam {
    pub fn default() -> Self {
        InputParam { data_source: DataSource::TEXT(String::from("<div name=\"item1\">Canada</div>")), node_selector: NodeSelector::Css(String::from("div")) }
    }

    fn new(data_source: DataSource, node_selector: NodeSelector) -> Self {
        InputParam { data_source, node_selector }
    }
}

impl TryFrom<Args> for InputParam {
    type Error = std::io::Error;
    fn try_from(mut args: Args) -> std::result::Result<Self, Self::Error> {
       //[--url|u URL] <--xpath XPATH | --css CSS-SELECTOR>\n
       //      --xpath | -x XPATH. xpath to select the document node
       //      --css | -c SELECTOR select the document node 
       let mut kv = HashMap::<String, String>::new();
       while let Some(key) = args.next() {
           if !key.starts_with("-") {
               continue;
           }
           let key = &key[1..];
           let key = if key.starts_with("-") {
               &key[1..]
           } else {
               key
           };

           if let Some(val) = args.next() {
               kv.insert(String::from(key), String::from(val));
           }
       }
       let data_source: DataSource = if kv.contains_key("url") {
           DataSource::URL(kv["url"].clone())
       } else if kv.contains_key("u") {
           DataSource::URL(kv["u"].clone())
       } else {
           DataSource::TEXT(std::io::stdin().lines().map(|v| v.unwrap()).collect::<Vec<String>>().join("\n"))
       };

       let node_selector = if kv.contains_key("css") {
           NodeSelector::Css(kv["css"].clone())
       } else if kv.contains_key("c") {
           NodeSelector::Css(kv["c"].clone())
       } else if kv.contains_key("xpath") {
           NodeSelector::Xpath(kv["xpath"].clone())
       }  else if kv.contains_key("x") {
           NodeSelector::Xpath(kv["x"].clone())
       } else {
           return Err(Error::new(ErrorKind::InvalidInput, "Not found neither --css nor --xpath"))
       };

       Ok(InputParam{ data_source, node_selector })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataSource {
    URL(String),
    TEXT(String)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_css() {
        let input = InputParam::default();
        let query_result = query(&input);
        assert!(query_result.is_ok());
        let query_result = query_result.unwrap();
        assert_eq!(query_result, "Canada");
    }

    #[test]
    fn test_query_css_multi_result() {
        let input = InputParam::new(DataSource::TEXT(String::from("<li><div>Toronto</div><div>Ottawa</div></li>")), NodeSelector::Css(String::from("div")));
        let query_result = query(&input);
        assert!(query_result.is_ok());
        let query_result = query_result.unwrap();
        assert_eq!(query_result, "Toronto\nOttawa");
    }

    #[test]
    fn test_parse_input() {
        let input = InputParam::try_from(String::from("--url https://www.google.com --css div")).expect("failed to parse input");
        assert_eq!(input.data_source, DataSource::URL(String::from("https://www.google.com")));
    }

    #[test]
    fn test_parse_input_short() {
        let input = InputParam::try_from(String::from("-u https://www.google.com -x /div")).expect("failed to parse input");
        assert_eq!(input.data_source, DataSource::URL(String::from("https://www.google.com")));
    }

    #[test]
    fn test_query_xpath() {
        let input = InputParam::new(DataSource::TEXT(String::from("<li><div>Toronto</div><div>Ottawa</div></li>")), NodeSelector::Xpath(String::from("//div")));
        let query_result = query(&input);
        let query_result = query_result.unwrap();
        assert_eq!(query_result, "Toronto\nOttawa");
    }

}
