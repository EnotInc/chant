use std::path::Path;
use pathdiff::diff_paths;

/// About |create_obj_link()|
/// Small helper func, to create markdown link
pub fn create_obj_link(obj: &str, index: i32, path: &str, relative: Option<String>) -> String {
    match relative {
        Some(v) => {
            let base = Path::new(path).parent().unwrap();
            let target = Path::new(&v);
            let relative = diff_paths(target, base).unwrap();

            let link = format!("{}({}#L{})", obj, relative.to_string_lossy(), index);
            return link;
        }
        None => {
            let filename = Path::new(path).file_name().unwrap().to_string_lossy().to_string();
            let link = format!("[{}]({}#L{})", obj, filename, index);
            return link;
        }
    }
}