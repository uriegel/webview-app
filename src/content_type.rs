pub fn get(uri: &str)->String { 
    let uri_lower = uri.to_lowercase();
    if uri_lower.ends_with(".html") {
        "text/html"
    } else if uri_lower.ends_with(".css") {
        "text/css"
    } else if uri_lower.ends_with(".js") {
        "application/javascript"
    } else if uri_lower.ends_with(".jpg") {
        "image/jpeg"
    } else if uri_lower.ends_with(".png") {
        "image/png"
    } else if uri_lower.ends_with(".svg") {
        "image/svg+xml"
    }
    else {
        "text/text"
    }.to_string()
}