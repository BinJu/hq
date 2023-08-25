use std::io::Result;
use scraper::{Html, Selector};

enum CSSSelector {
    TEXT(String),
    PROP(String, String)
}

impl From<&str> for CSSSelector {
    fn from(str_selector: &str) -> Self {
        parse_css_selector(str_selector)
    }
}

impl From<&String> for CSSSelector {
    fn from(str_selector: &String) -> Self {
        parse_css_selector(str_selector)
    }
}

fn parse_css_selector(selector: impl Into<String>) -> CSSSelector {
    let selector_str = selector.into();
    let selector_piecies: Vec<String> = selector_str.split("@").map(|v| String::from(v)).collect();
    if selector_piecies.len() > 1 {
        CSSSelector::PROP(selector_piecies[0].clone(), selector_piecies[1].clone())
    } else {
        CSSSelector::TEXT(selector_str)
    }
}

pub(super) fn query(css_selector: &String, doc: String) -> Result<Vec<String>> {
    let fragment = Html::parse_fragment(doc.as_str());
    match CSSSelector::from(css_selector) {
        CSSSelector::TEXT(selector) => {
            let css_selector = Selector::parse(selector.as_str()).unwrap();
            let mut result: Vec<String> = Vec::new();
            for element in fragment.select(&css_selector) {
                let value = element.text().collect();
                result.push(value);
            }
            Ok(result)
        },
        CSSSelector::PROP(selector, prop) => {
            let css_selector = Selector::parse(selector.as_str()).unwrap();
            let mut result: Vec<String> = Vec::new();
            for element in fragment.select(&css_selector) {
                match element.value().attr(prop.as_str()) {
                    Some(v) => result.push(String::from(v)),
                    None => {}
                }
            }
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_query() {
        let doc = String::from("<li><div name=\"first\">Toronto</div><div name=\"second\">North York</div></li>");
        let items = query(&String::from("div"), doc).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], String::from("Toronto"));
        assert_eq!(items[1], String::from("North York"));
    }

    #[test]
    fn test_css_query_property() {
        let doc = String::from("<li><div name=\"first\">Toronto</div><div name=\"second\">North York</div></li>");
        let items = query(&String::from("div@name"), doc).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], String::from("first"));
        assert_eq!(items[1], String::from("second"));
    }
}
