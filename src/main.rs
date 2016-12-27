use std::env;
use std::fs;
use std::path::Path;
use std::convert::AsRef;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 1 {
        exit(1)
    }

    let mut all = false;
    let mut bool_only = false;
    let mut file = String::new();

    for arg in args.iter().skip(1) {
        match &arg[..] {
            "-a" => all = true,
            "-s" => bool_only = true,
            name => file.push_str(name),
        };
    }

    if file.is_empty() {
        println!("empty");
        exit(1)
    }

    let paths = paths_for_executable(file.as_ref());

    if bool_only {
        if paths.is_empty() {
            exit(1);
        }

        exit(0);
    }


    if paths.is_empty() {
        println!("{} not found", file);
        exit(0);
    }

    if all {
        for path in paths {
            println!("{}", path);
        }

        exit(0);
    }

    println!("{}", paths[0]);

}

fn paths_for_executable(executable: &str) -> Vec<String> {
    let mut vec = Vec::new();
    let path = env::var("PATH").expect("No PATH environment variable set");
    let directories = path.split(":").map(Path::new).collect::<Vec<&Path>>();

    for directory in &directories {
        match fs::metadata(directory) {
            Ok(meta) => {
                if !meta.is_dir() {
                    continue;
                }
            }
            Err(_) => continue,
        };

        let paths = fs::read_dir(directory).unwrap();

        for path in paths {
            let file = path.unwrap().path();

            if let Some(name) = file.file_name() {
                if executable == name.to_str().unwrap() {
                    vec.push(file.to_str().unwrap().to_string());
                }
            }
        }
    }

    vec
}
