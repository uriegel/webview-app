pub fn get(uri: &str)->String { 
    if uri.ends_with(".html") {
        "text/html"
    } else if uri.ends_with(".css") {
        "text/css"
    } else if uri.ends_with(".js") {
        "application/javascript"
    } else if uri.ends_with(".svg") {
        "image/svg+xml"
    }
    else {
        "text/text"
    }.to_string()
}