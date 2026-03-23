use anyhow::Result;
use std::path::Path;

use crate::scanner::{UnsafeOccurrence, UnsafeKind};

/**
 * Rust syntax parser for detecting and analyzing unsafe code blocks and declarations
 */

pub struct UnsafeParser;

impl UnsafeParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse(&self, content: &str, _path: &Path) -> Result<Vec<UnsafeOccurrence>> {
        let mut occurrences = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed.starts_with("unsafe fn") || trimmed.starts_with("pub unsafe fn") {
                occurrences.push(UnsafeOccurrence {
                    line: line_number,
                    column: line.find("unsafe").unwrap_or(0) + 1,
                    kind: UnsafeKind::Function,
                    context: Some(trimmed.to_string()),
                });
            }
            
            if trimmed.starts_with("unsafe {") || trimmed == "unsafe" || trimmed.contains("= unsafe {") {
                occurrences.push(UnsafeOccurrence {
                    line: line_number,
                    column: line.find("unsafe").unwrap_or(0) + 1,
                    kind: UnsafeKind::Block,
                    context: Some(trimmed.to_string()),
                });
            }
            
            if trimmed.starts_with("unsafe trait") || trimmed.starts_with("pub unsafe trait") {
                occurrences.push(UnsafeOccurrence {
                    line: line_number,
                    column: line.find("unsafe").unwrap_or(0) + 1,
                    kind: UnsafeKind::Trait,
                    context: Some(trimmed.to_string()),
                });
            }
            
            if trimmed.starts_with("unsafe impl") {
                occurrences.push(UnsafeOccurrence {
                    line: line_number,
                    column: line.find("unsafe").unwrap_or(0) + 1,
                    kind: UnsafeKind::Impl,
                    context: Some(trimmed.to_string()),
                });
            }
            
            if trimmed.starts_with("unsafe extern") || trimmed.starts_with("extern \"C\"") {
                if trimmed.contains("unsafe") || (line_num > 0 && lines[line_num - 1].trim().contains("unsafe")) {
                    occurrences.push(UnsafeOccurrence {
                        line: line_number,
                        column: line.find("unsafe").or_else(|| line.find("extern")).unwrap_or(0) + 1,
                        kind: UnsafeKind::Extern,
                        context: Some(trimmed.to_string()),
                    });
                }
            }
        }
        
        Ok(occurrences)
    }
}