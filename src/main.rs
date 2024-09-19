use std::{io, str::from_utf8};
use curl::easy::{Easy, List};

const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyz1234567890";
const MIN_LEN: usize = 20;
const MAX_LEN: usize = 50;

fn main() {
    let mut result = Vec::<String>::with_capacity(20);

    let mut visited_buff = Vec::<String>::with_capacity(100);
    visited_buff.push("https://web.evanchen.cc/index.html".to_string());

    let mut to_visit_buff = Vec::<String>::with_capacity(50);

    let mut current_page = "https://web.evanchen.cc/".to_string();
    loop {
        let current_page_body = get_page_body(&current_page);
        let mut link = current_page.to_string();

        for word in current_page_body.trim().split_ascii_whitespace() {
            // Check if word contains a link
            if word.contains("https://web.evanchen.cc/") 
                && word.contains(".html") {
                
                // Get link from word
                link = word
                    .to_string()
                    .split_off(word.find("https://").unwrap());
                let _ = link.split_off(link.find(".html").unwrap() + 5);

                // Add link to stack if needed
                if !to_visit_buff.contains(&link) && !visited_buff.contains(&link) {
                    to_visit_buff.push(link.clone());
                }
            }

            // Chack if word contains a hash
            match get_hash(word) {
                Ok(hashes) => {
                    for hash in hashes {
                        if !result.contains(&hash) {
                            println!("{} from {}", &hash, &link);
                            result.push(hash);
                        }
                    }
                },
                Err(_) => {}
            }
        }

        // Set current page as visited and get next page
        visited_buff.push(current_page);
        match to_visit_buff.pop() {
            Some(x) => current_page = x,
            None => break
        }
    }
    
    // TODO write result to file
    
    println!("Press any key to close...");
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}

// Gets body of a web page from a url // TODO circumvent crawl protection of some hosts
fn get_page_body(url: &str) -> String {
    let mut page = String::default();

    let header = List::new();

    let mut handle = Easy::new();
    handle.follow_location(true).unwrap();
    handle.http_headers(header).unwrap();
    handle.url(url).unwrap();

    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        match from_utf8(data) {
            Ok(body) => page = body.to_string(),
            Err(_) => {
                eprintln!("Page \"{}\" too big or not UTF8; Skiping", url);
                page = "<body></body>".to_string();
            }
        }
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    let margin: [Option<usize>; 2] = [page.find("</body>"), page.find("<body>")];
    if margin[0].is_some() && margin[1].is_some() {
        page = page.split_off(page.find("<body>").expect("Page has no body tag"));
        let _ = page.split_off(page.find("</body>").expect("Page has no end body tag") + 7);
        return page;
    }

    //eprintln!("{}", url);
    get_page_body(url)
}

// Check and extract possible hashes from words
fn get_hash(s: &str) -> Result<Vec<String>, ()> {
    let mut result = Vec::<String>::with_capacity(8);

    // Hash -> continous substring that includes only letters and numbers
    let mut potential_hash = String::new();
    for ch in s.chars() {
        if ALLOWED_CHARS.contains(ch) && potential_hash.len() < MAX_LEN { potential_hash.push(ch); }
        else {
            if potential_hash.len() >= MIN_LEN && potential_hash.len() <= MAX_LEN {
                result.push(potential_hash.clone());
                potential_hash.clear();
                
                if ALLOWED_CHARS.contains(ch) {
                    potential_hash.push(ch);
                }
            }
            else {
                potential_hash.clear();

                if ALLOWED_CHARS.contains(ch) {
                    potential_hash.push(ch);
                }
            }
        }
    }

    if result.is_empty() { return Err(()); }
    Ok(result)
}