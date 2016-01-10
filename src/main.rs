use std::env;
use std::fs;
use std::path::Path;


fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 1 {
        return;
    }
    let path = env::var("PATH").expect("No PATH environment variable set");
    let directories = path.split(":").map(Path::new).collect::<Vec<&Path>>();

    for directory in &directories {
        match fs::metadata(directory) {
            Ok(meta) => if !meta.is_dir() { continue; },
            Err(_)   => continue,
        };

        let paths = fs::read_dir(directory).unwrap();

        for path in paths {
            let file = path.unwrap().path();

            if let Ok(f) = fs::metadata(&file) {
                if !f.is_file() {
                    continue;
                }

                if let Some(name) = file.file_name() {
                    println!("{}", name.to_str().unwrap());
                }

            }

        }

    }
}
