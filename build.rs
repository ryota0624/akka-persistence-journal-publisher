use std::fs;
use std::io;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

fn find_file(dir: &Path, check: &dyn Fn(&Path) -> bool) -> io::Result<Vec<PathBuf>> {
    if dir.is_dir() {
        let mut dir = fs::read_dir(dir)?;
        dir.try_fold(Vec::new(), |paths: Vec<_>, entry| {
            let entry = entry?;
            let path = entry.path();
            let files = find_file(path.as_path(), check)?;
            Ok([paths, files].concat())
        })
    } else {
        let files: Vec<_> = if check(dir) {
            vec![dir.to_path_buf()]
        } else {
            vec![]
        };
        Ok(files)
    }
}

fn main() -> Result<()> {
    let proto_files = find_file(Path::new("src/proto"), &|path| {
        let ext = path.extension().map(std::ffi::OsStr::to_str).flatten();
        ext == Some("proto")
    })?;
    prost_build::compile_protos(&proto_files, &["src/"])?;

    Ok(())
}
