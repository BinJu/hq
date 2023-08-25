use std::io::{Result, Error, ErrorKind};

use skyscraper::{html, xpath};

enum XPathSelector {
    TEXT(String),
    PROP(String, String)
}

impl From<&str> for XPathSelector {
    fn from(str_selector: &str) -> Self {
        parse_xpath_selector(str_selector)
    }
}

impl From<&String> for XPathSelector {
    fn from(str_selector: &String) -> Self {
        parse_xpath_selector(str_selector)
    }
}


fn parse_xpath_selector(selector: impl Into<String>) -> XPathSelector {
    let selector_str = selector.into();
    let selector_piecies: Vec<String> = selector_str.split("@").map(|v| String::from(v)).collect();
    if selector_piecies.len() > 1 {
        XPathSelector::PROP(selector_piecies[0].clone(), selector_piecies[1].clone())
    } else {
        XPathSelector::TEXT(selector_str)
    }
}


pub(super) fn query(xpath_selector: &String, doc: String) -> Result<Vec<String>> {
    let document = html::parse(&*doc).map_err(|e| Error::new(ErrorKind::InvalidInput, format!("failed to parse doc {}. Error: {}", doc, e)))?;
 
    match XPathSelector::from(xpath_selector) {
        XPathSelector::TEXT(selector) => {
            // Parse and apply the xpath.
            let expr = xpath::parse(&selector).map_err(|e| Error::new(ErrorKind::InvalidInput, format!("failed to parse xpath expression {}. Error: {}", xpath_selector, e)))?;
            let elements = expr.apply(&document).map_err(|e| Error::new(ErrorKind::InvalidInput, format!("failed to parse doc {}. Error: {}", doc, e)))?;
            let mut result: Vec<String> = Vec::new();
            for element in &elements {
                let text = element.get_text(&document).map_or(String::new(), |x|x);
                result.push(text);
            }
            Ok(result)
        },
        XPathSelector::PROP(selector, prop) => {
            // Parse and apply the xpath.
            let expr = xpath::parse(&selector).map_err(|e| Error::new(ErrorKind::InvalidInput, format!("failed to parse xpath expression {}. Error: {}", xpath_selector, e)))?;
            let elements = expr.apply(&document).map_err(|e| Error::new(ErrorKind::InvalidInput, format!("failed to parse doc {}. Error: {}", doc, e)))?;
            let mut result: Vec<String> = Vec::new();
            for element in &elements {
                if let Some(attributes) = element.get_attributes(&document)  {
                    result.push(attributes[&prop].clone());
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
    fn test_xpath_query() {
        let result = query(&String::from("/div"), String::from("<div>Toronto</div>")).unwrap();
        assert_eq!(result.len(), 1);
    }
}
