//! Government kernel shader management for SigmaOS.
//!
//! Provides shader compilation, rendering pipeline, and graphics
//! resource management. Simulates GPU shader operations without actual
//! hardware graphics.

use std::collections::HashMap;
use std::fmt;

/// Errors for shader operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderError {
    ShaderNotFound(String),
    CompileFailed(String),
    InvalidSource(String),
    LinkFailed(String),
    ResourceExhausted(String),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShaderNotFound(s) => write!(f, "shader not found: {s}"),
            Self::CompileFailed(m) => write!(f, "compile failed: {m}"),
            Self::InvalidSource(m) => write!(f, "invalid source: {m}"),
            Self::LinkFailed(m) => write!(f, "link failed: {m}"),
            Self::ResourceExhausted(m) => write!(f, "resource exhausted: {m}"),
        }
    }
}

impl std::error::Error for ShaderError {}

/// Shader type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
    Geometry,
    Tessellation,
}

impl ShaderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Fragment => "fragment",
            Self::Compute => "compute",
            Self::Geometry => "geometry",
            Self::Tessellation => "tessellation",
        }
    }
}

/// Shader source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShaderSource {
    pub name: String,
    pub shader_type: ShaderType,
    pub source: String,
    pub version: String,
}

impl ShaderSource {
    pub fn new(name: impl Into<String>, shader_type: ShaderType, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            shader_type,
            source: source.into(),
            version: "1.0".to_string(),
        }
    }

    pub fn validate(&self) -> Result<(), ShaderError> {
        if self.name.trim().is_empty() {
            return Err(ShaderError::InvalidSource("name must not be empty".into()));
        }
        if self.source.trim().is_empty() {
            return Err(ShaderError::InvalidSource("source must not be empty".into()));
        }
        Ok(())
    }
}

/// Compiled shader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledShader {
    pub id: String,
    pub source: ShaderSource,
    pub compiled: bool,
    pub binary_size: usize,
}

/// Shader program (vertex + fragment).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShaderProgram {
    pub id: String,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub linked: bool,
}

/// Government kernel shader manager.
#[derive(Debug, Clone, Default)]
pub struct ShaderManager {
    shaders: HashMap<String, CompiledShader>,
    programs: HashMap<String, ShaderProgram>,
    next_id: u64,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(&mut self, source: ShaderSource) -> Result<String, ShaderError> {
        source.validate()?;
        
        self.next_id += 1;
        let id = format!("shader-{}", self.next_id);
        let binary_size = source.source.len() * 2; // Simulate compiled size
        
        let compiled = CompiledShader {
            id: id.clone(),
            source: source.clone(),
            compiled: true,
            binary_size,
        };
        self.shaders.insert(id.clone(), compiled);
        Ok(id)
    }

    pub fn get_shader(&self, id: &str) -> Option<&CompiledShader> {
        self.shaders.get(id)
    }

    pub fn remove_shader(&mut self, id: &str) -> Result<(), ShaderError> {
        self.shaders.remove(id)
            .map(|_| ())
            .ok_or_else(|| ShaderError::ShaderNotFound(id.to_string()))
    }

    pub fn link_program(&mut self, vertex_id: &str, fragment_id: &str) -> Result<String, ShaderError> {
        let vertex = self.shaders.get(vertex_id)
            .ok_or_else(|| ShaderError::ShaderNotFound(vertex_id.to_string()))?;
        let fragment = self.shaders.get(fragment_id)
            .ok_or_else(|| ShaderError::ShaderNotFound(fragment_id.to_string()))?;

        if vertex.source.shader_type != ShaderType::Vertex {
            return Err(ShaderError::LinkFailed("vertex shader must be vertex type".into()));
        }
        if fragment.source.shader_type != ShaderType::Fragment {
            return Err(ShaderError::LinkFailed("fragment shader must be fragment type".into()));
        }

        self.next_id += 1;
        let program_id = format!("program-{}", self.next_id);
        let program = ShaderProgram {
            id: program_id.clone(),
            vertex_shader: vertex_id.to_string(),
            fragment_shader: fragment_id.to_string(),
            linked: true,
        };
        self.programs.insert(program_id.clone(), program);
        Ok(program_id)
    }

    pub fn get_program(&self, id: &str) -> Option<&ShaderProgram> {
        self.programs.get(id)
    }

    pub fn list_shaders(&self) -> Vec<&CompiledShader> {
        self.shaders.values().collect()
    }

    pub fn list_programs(&self) -> Vec<&ShaderProgram> {
        self.programs.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_shader_aaa() {
        // Arrange
        let mut mgr = ShaderManager::new();
        let source = ShaderSource::new("vertex", ShaderType::Vertex, "void main() { gl_Position = vec4(0.0); }");

        // Act
        let id = mgr.compile(source);

        // Assert
        assert!(id.is_ok());
        assert_eq!(mgr.list_shaders().len(), 1);
    }

    #[test]
    fn test_link_program_aaa() {
        // Arrange
        let mut mgr = ShaderManager::new();
        let vertex = ShaderSource::new("vertex", ShaderType::Vertex, "void main() {}");
        let fragment = ShaderSource::new("fragment", ShaderType::Fragment, "void main() {}");
        let vid = mgr.compile(vertex).unwrap();
        let fid = mgr.compile(fragment).unwrap();

        // Act
        let program_id = mgr.link_program(&vid, &fid);

        // Assert
        assert!(program_id.is_ok());
        assert_eq!(mgr.list_programs().len(), 1);
    }

    #[test]
    fn test_shader_not_found_aaa() {
        // Arrange
        let mgr = ShaderManager::new();

        // Act
        let res = mgr.get_shader("nonexistent");

        // Assert
        assert!(res.is_none());
    }
}
