use walkdir::WalkDir;

fn main() {
    let mut files = vec![
        String::from("src/main.c"),
    ];

    for entry in WalkDir::new(".c").into_iter().filter_map(|e| e.ok()) {
        if let Some(i) = entry.path().to_str() {
            files.push(String::from(i));
        }
    }

    for file in files.iter() {
        println!("cargo:rerun-if-changed={}", file);
    }

    cc::Build::new()
        .files(files.iter())
        .compile("main");
}