use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "virt-install-rs")]
#[command(about = "A minimal virt-install clone in Rust", long_about = None)]
pub struct CliArgs {
    /// Name of the virtual machine
    #[arg(long)]
    pub name: String,

    /// Memory size in MiB
    #[arg(long)]
    pub memory: u64,

    /// Number of virtual CPUs
    #[arg(long)]
    pub vcpus: u32,

    /// Path to the disk image
    #[arg(long)]
    pub disk_path: String,

    /// Path to the CD-ROM ISO (optional)
    #[arg(long)]
    pub cdrom_path: Option<String>,

    /// Only output the generated domain XML, do not create the VM
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}
