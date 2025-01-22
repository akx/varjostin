use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Default, Debug)]
pub(crate) struct FileCollection {
    pub root: PathBuf,
    pub suffixes: Vec<String>,
    pub files: Vec<(String, PathBuf)>,
}

impl FileCollection {
    pub fn new(root: &PathBuf, patterns: &[&str]) -> Self {
        Self {
            root: root.clone(),
            suffixes: patterns.iter().map(|p| p.to_string()).collect(),
            files: Vec::new(),
        }
    }

    pub fn collect_files(&mut self) -> eyre::Result<usize> {
        let mut files: Vec<(String, PathBuf)> = Vec::new();
        for entry in WalkDir::new(&self.root) {
            if let Ok(entry) = entry {
                if !entry.file_type().is_file() {
                    continue;
                }
                let name = entry.file_name().to_string_lossy();
                if !self.suffixes.iter().any(|suffix| name.ends_with(suffix)) {
                    continue;
                }
                files.push((name.parse()?, PathBuf::from(entry.path())));
            }
        }
        self.files = files;
        Ok(self.files.len())
    }
}
