use scraper;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Website to scrap
    let url: String = "https://regsystem.bunkalang.com/index.php?route=lesson/course&path=20_1".into();
    
    // Response after scraping
    let res = reqwest::blocking::get(url)?;

    // Get the body of the response
    let body = res.text()?;

    // Convert HTML string into document type
    let document = Html::parse_document(&body);

    // Specify the class that we are looking for
    let selector = Selector::parse("a").unwrap();

    // Print out the elements
    let s = document.select(&selector);

    let mut f = s.filter(|s| s.inner_html().contains("*IN-PERSON*"));

    while let Some(node) = f.next() {
        println!("{:?}", node.inner_html());
    }

    Ok(())
}