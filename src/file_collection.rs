use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Default, Debug)]
pub(crate) struct FileCollection {
    pub root: PathBuf,
    pub suffixes: Vec<String>,
    pub files: Vec<(String, PathBuf)>,
}

impl FileCollection {
    pub fn new(root: &Path, patterns: &[&str]) -> Self {
        Self {
            root: root.to_path_buf(),
            suffixes: patterns.iter().map(|p| p.to_string()).collect(),
            files: Vec::new(),
        }
    }

    pub fn collect_files(&mut self) -> eyre::Result<usize> {
        let mut files: Vec<(String, PathBuf)> = Vec::new();
        for entry in WalkDir::new(&self.root).into_iter().flatten() {
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy();
            if !self.suffixes.iter().any(|suffix| name.ends_with(suffix)) {
                continue;
            }
            files.push((name.parse()?, PathBuf::from(entry.path())));
        }
        files.sort_by(|a, b| a.0.cmp(&b.0));
        self.files = files;
        Ok(self.files.len())
    }
}
