use headless_chrome::Browser;
use std::fs;

fn main() {
    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to("https://tivolicloud.com").unwrap();
    tab.wait_until_navigated().unwrap();
    let pdf_data = tab.print_to_pdf(None).unwrap();
    fs::write("tivolicloud.pdf", &pdf_data).unwrap();
}
