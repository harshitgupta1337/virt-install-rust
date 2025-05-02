use clap::Parser;
use std::collections::HashMap;
use log::info;

pub const FIRMWARE_TYPE_KEY: &str = "firmwareType";

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

    /// Boot options in the format [<firmwareType>,]option1=val1,option2=val2,...
    #[arg(long, value_parser = parse_boot_options)]
    pub boot: HashMap<String, String>,

    /// Only output the generated domain XML, do not create the VM
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

/// Parses the boot options in the format `[<firmwareType>,]option1=val1,option2=val2,...`
pub fn parse_boot_options(value: &str) -> Result<HashMap<String, String>, String> {
    let mut result = HashMap::new();

    // Split the input into firmware type and options
    let parts: Vec<&str> = value.splitn(2, ',').collect();
    result.insert(FIRMWARE_TYPE_KEY.to_string(), parts[0].to_string());
    if parts.len() == 2 {
        // Parse the key-value pairs
        let options = parts.last().unwrap_or(&value);
        for pair in options.split(',') {
            let kv: Vec<&str> = pair.splitn(2, '=').collect();
            if kv.len() == 2 {
                result.insert(kv[0].to_string(), kv[1].to_string());
                info!("result = {:#?}", result);
            } else {
                return Err(format!("Invalid boot option format: {}", pair));
            }
        }
    }

    Ok(result)
}