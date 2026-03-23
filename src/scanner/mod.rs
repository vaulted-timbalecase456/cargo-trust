use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::error::TrustError;
use crate::parser::UnsafeParser;

pub use self::results::{ScanResult, UnsafeOccurrence, UnsafeKind};

mod results;

/**
 * File system scanner responsible for finding and analyzing Rust source files
 */

pub struct Scanner {
    root_path: PathBuf,
}

impl Scanner {
    pub fn new(path: PathBuf) -> Self {
        Self { root_path: path }
    }
    
    pub fn scan(&self) -> Result<ScanResult> {
        let mut result = ScanResult::new(self.root_path.clone());
        
        for entry in WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if self.should_skip(path) {
                continue;
            }
            
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                match self.scan_file(path) {
                    Ok(occurrences) => {
                        if !occurrences.is_empty() {
                            result.add_file(path.to_path_buf(), occurrences);
                        }
                    }
                    Err(e) => {
                        result.add_error(path.to_path_buf(), e.to_string());
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    fn scan_file(&self, path: &Path) -> Result<Vec<UnsafeOccurrence>> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| TrustError::FileRead {
                path: path.to_path_buf(),
                source: e,
            })?;
        
        let parser = UnsafeParser::new();
        parser.parse(&content, path)
    }
    
    fn should_skip(&self, path: &Path) -> bool {
        path.components().any(|c| {
            c.as_os_str() == "target" || 
            c.as_os_str() == ".git" ||
            c.as_os_str() == "vendor" ||
            c.as_os_str().to_string_lossy().starts_with('.')
        })
    }
}