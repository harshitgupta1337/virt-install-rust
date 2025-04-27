mod cli;
mod domain;
mod error;
mod libvirt;

use clap::Parser;
use cli::CliArgs;
use domain::generate_domain_xml;
use libvirt::create_and_start_domain;
use log::info;

fn main() {
    env_logger::init();
    let args = CliArgs::parse();

    info!("Parsed CLI arguments: {:?}", args);

    let domain_xml = generate_domain_xml(&args);
    info!("Generated domain XML:\n{}", domain_xml);

    if args.dry_run {
        info!("Exiting w/o creating domain because --dry-run was specified.")
    } else {
        match create_and_start_domain(&domain_xml) {
            Ok(_) => println!("VM '{}' created and started successfully.", args.name),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}