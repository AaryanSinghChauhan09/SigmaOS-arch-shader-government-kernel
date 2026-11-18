// Integration test for SigmaOS-arch-shader-government-kernel
// Verifies the crate builds and links as a dependency.

use sigmaos_arch_shader_government_kernel::{ShaderManager, ShaderSource, ShaderType};

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut mgr = ShaderManager::new();
    let source = ShaderSource::new("vertex", ShaderType::Vertex, "void main() {}");

    // Act
    let res = mgr.compile(source);

    // Assert
    assert!(res.is_ok());
    assert_eq!(mgr.list_shaders().len(), 1);
}
