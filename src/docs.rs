use colored::*;

pub fn docs() {
    println!(
        "{}",
        "🔧 Aginisi — Fast JSON-Backed Mock API Server"
            .yellow()
            .bold()
    );
    println!("{}", "📂 Project Structure".blue().bold());
    println!(
        "{}",
        " ├── aginisi/
 │ ├── users.json
 │ └── products.json
 ├── aginisi.toml # optional config"
    );
    println!("{}", "## TO Run".blue().bold());
    println!(
        "{}",
        " - aginisi serve
 - aginisi serve -- --port 5000
 - aginisi serve -- --docs
 - aginisi serve -- --path . # to specify the part to run aginisi
    "
    )
}
