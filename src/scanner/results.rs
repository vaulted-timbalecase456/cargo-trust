use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/**
 * Data structures for storing and organizing scan results
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub project_root: PathBuf,
    pub files_with_unsafe: HashMap<PathBuf, Vec<UnsafeOccurrence>>,
    pub total_unsafe_count: usize,
    pub errors: Vec<ScanError>,
}

impl ScanResult {
    pub fn new(root: PathBuf) -> Self {
        Self {
            project_root: root,
            files_with_unsafe: HashMap::new(),
            total_unsafe_count: 0,
            errors: Vec::new(),
        }
    }
    
    pub fn add_file(&mut self, path: PathBuf, occurrences: Vec<UnsafeOccurrence>) {
        self.total_unsafe_count += occurrences.len();
        self.files_with_unsafe.insert(path, occurrences);
    }
    
    pub fn add_error(&mut self, path: PathBuf, message: String) {
        self.errors.push(ScanError { path, message });
    }
    
    pub fn is_empty(&self) -> bool {
        self.files_with_unsafe.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsafeOccurrence {
    pub line: usize,
    pub column: usize,
    pub kind: UnsafeKind,
    pub context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnsafeKind {
    Block,
    Function,
    Trait,
    Impl,
    Extern,
}

impl std::fmt::Display for UnsafeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block => write!(f, "unsafe block"),
            Self::Function => write!(f, "unsafe function"),
            Self::Trait => write!(f, "unsafe trait"),
            Self::Impl => write!(f, "unsafe impl"),
            Self::Extern => write!(f, "unsafe extern"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanError {
    pub path: PathBuf,
    pub message: String,
}