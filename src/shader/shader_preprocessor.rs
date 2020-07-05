use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str;

use regex::Regex;

pub fn shader_preprocessor(shader_path: &str) -> String {
    let mut included_files = HashSet::new();
    let ret = preprocessor_helper(shader_path, &mut included_files);
    ret
}

fn preprocessor_helper(shader_path: &str, included_files: &mut HashSet<String>) -> String {
    // Adds a file path to the included_files path
    //   Returns the file
    let mut shader_file = File::open(shader_path).unwrap_or_else(|_| panic!("Failed to open {}", shader_path));
    let mut shader_body = String::new();
    shader_file
        .read_to_string(&mut shader_body)
        .expect("Failed to read shader body");
    let mut ret = shader_body.clone();
    let re = Regex::new("#include \"([a-z./]+)\"").unwrap();
    for import_file in re.captures_iter(&shader_body) {
        let inc_file = String::from(&import_file[1]);
        included_files.insert(inc_file.clone());
        let inc_body = preprocessor_helper(&inc_file, included_files);
        ret = ret.replace(&import_file[0], &inc_body); // Will fail because ret is different from shader_body
    }
    ret
}
