use promptly::prompt_opt;
use std::path::Path;
use std::process::{Command, Output};

fn main() {
    let directory = std::env::args().last().unwrap_or(".".to_string());
    let output = Command::new("find")
        .args([&directory, "-xtype", "l", "-print0"])
        .output()
        .expect("Failed to run GNU find utility (is it installed and in $PATH?)");
    let broken_links = parse_find_output(&output);

    for link in broken_links {
        let moved_candidates = find_moved(&directory, &link);
        if moved_candidates.len() > 0 {
            println!("{}: Found possible moved file(s).", link);
            for (i, candidate) in moved_candidates.iter().enumerate() {
                println!("\t[{}] {}", i, candidate);
            }
            let index: Option<usize> =
                prompt_opt("Enter a file to relink to or nothing to skip").expect("readline error");
            match index {
                Some(i) if i < moved_candidates.len() => {
                    symlink(&moved_candidates[i], &link);
                }
                _ => {}
            }
        } else {
            println!("{}: No files found with the same name", link);
        }
    }
}

fn find_moved(directory: &str, link: &str) -> Vec<String> {
    let basename = Path::new(link)
        .file_name()
        .expect(&format!("Could not parse filename from {:?}", link))
        .to_str()
        .expect(&format!("Could not convert OsStr to str for {:?}", link));
    let output = Command::new("find")
        .args([directory, "-type", "f", "-name", basename, "-print0"])
        .output()
        .expect("Failed to run GNU find utility (is it installed and in $PATH?)");
    return parse_find_output(&output);
}

fn parse_find_output(output: &Output) -> Vec<String> {
    std::str::from_utf8(&output.stdout)
        .expect("Received invalid UTF-8 from find utility")
        .split_terminator("\0")
        .map(|v| v.to_owned())
        .collect()
}

fn symlink(target: &str, link_name: &str) {
    Command::new("ln")
        .args(["-rsvf", "-T", target, link_name])
        .status()
        .expect("Failed to execute ln (is it installed and in $PATH?)");
}
