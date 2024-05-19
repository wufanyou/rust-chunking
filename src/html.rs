use ammonia::Builder;
use regex::Regex;
use html2text::from_read;
use mdka::from_html;

pub(crate) fn convert(input:&str) -> String{

    let html = Builder::default()
                   .rm_generic_attributes(&["src", "href"])
                   .rm_tags(&["a", "img", "div", "span" , "p", "script", "noscript", "footer", "nav"])
                   .link_rel(None)
                   .clean(input).to_string();


    let re = Regex::new(r"&gt;").unwrap();
    // let trimmed_text = trimmed_text.to_string();
    let trimmed_text = re.replace_all(html.as_str(), ">");
    let re = Regex::new(r"&lt;").unwrap();
    let trimmed_text = trimmed_text.to_string();
    let trimmed_text = re.replace_all(trimmed_text.as_str(), "<");

    let output = Builder::default()
               .rm_generic_attributes(&["src", "href"])
               .rm_tags(&["a", "img", "div", "span" , "p", "script", "noscript", "footer", "nav"])
               .link_rel(None)
                .clean(trimmed_text.as_ref()).to_string();
    let output = from_html(output.as_str());
    output
    // let re = Regex::new(r"\s+").unwrap();
    // let output = re.replace_all(output.as_str(), " ");
    // output.to_string()
}