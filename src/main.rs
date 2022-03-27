use select::document::Document;
use select::predicate::Name;
use url::Url;
use url::ParseError as UrlParseError;
fn main() -> eyre::Result<()>{
    let response = reqwest::blocking::get("https://www.e-casebook.com/")?;
    let base_url = response.url().clone();
    let body = response.text()?;

    let doc = Document::from(body.as_str());
    println!("body = {:?}", doc);
    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")){
        println!("{:?}",Url::parse(href));
        match Url::parse(href){
            Ok(url) => {println!("{}",url);},
            Err(UrlParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href)?;
                println!("{}",url);
            }
            Err(e) => {},
        }
    }

    Ok(())
}
