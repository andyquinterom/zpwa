use url::Url;

fn main() {
    let zoom_link = std::env::args().nth(1).expect("No zoom link provided");
    // Replace zoommtg:// with https://
    let zoom_link = zoom_link.replace("zoommtg://", "https://");
    let url = Url::parse(zoom_link.as_str()).expect("Invalid zoom link");
    let confno = url
        .query_pairs()
        .find(|(key, _)| key == "confno")
        .map(|(_, value)| value);
    match confno {
        Some(confno) => {
            let new_url = format!("https://app.zoom.us/wc/{confno}/join?fromPWA=1");
            webbrowser::open(new_url.as_str()).expect("Failed to open zoom");
        }
        None => {
            eprintln!("Invalid zoom link");
            std::process::exit(1);
        }
    }
}
