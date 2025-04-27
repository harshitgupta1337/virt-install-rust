use thiserror::Error;

#[derive(Error, Debug)]
pub enum VirtInstallError {
    #[error("Libvirt error: {0}")]
    Libvirt(#[from] virt::error::Error),

    #[error("Operation failed: {0}")]
    Operation(String),
}
