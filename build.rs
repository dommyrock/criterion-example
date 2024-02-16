use std::fs;
use std::path::Path;

fn main() {
    //(workaround to not re-run build rs every time /bin files are changed)
    //println!("cargo:rerun-if-changed=trigger.txt");

    //cleanup old
    let bin = Path::new("./src/bin");
    if bin.exists() {
        let _ = fs::remove_dir_all(bin);
    }

    //generate new
    let benches_path = Path::new("./benches");
    if benches_path.exists() {
        let benchmarks_path = benches_path.join("benchmarks");
        if benchmarks_path.exists() {
            let bin = Path::new("./src/bin");
            fs::create_dir_all(&bin).unwrap();

            for entry in fs::read_dir(benchmarks_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file()
                    && path.extension().unwrap() == "rs"
                    && path.file_name().unwrap() != "mod.rs"
                {
                    let content = fs::read_to_string(&path).unwrap();
                    let mut new_content = String::from(&content);
                    let file_name = path.file_name().unwrap();

                    // Check if the exec function has a lines parameter
                    if content.contains("fn exec(lines:Vec<&str>)") {
                        let input_path = Path::new("../../input.txt");
                        new_content.push_str(&format!("\n\nfn main() {{\n    let lines:Vec<_> =include_str!(\"{}\").lines().collect();\n    exec(lines);\n}}", input_path.display()));
                    } else {
                        new_content.push_str("\n\nfn main() {\n    exec();\n}");
                    }

                    let new_file_path = bin.join(file_name);
                    fs::write(new_file_path, new_content).unwrap();
                }
            }
        }
    }
}
