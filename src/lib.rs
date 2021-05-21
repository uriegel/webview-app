pub mod app;
mod mainwindow;
mod settings;

pub fn test() {
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    println!("Very early hello example test");
}

// TODO GTK: Extending Settings 
// TODO GTK: Properties like window title
// TODO Linux windows dependant sources
// TODO Windoes Window app
// TODO add webviews