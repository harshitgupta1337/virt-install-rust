use crate::error::VirtInstallError;
use virt::connect::Connect;
use virt::domain::Domain;

pub fn create_and_start_domain(xml: &str) -> Result<(), VirtInstallError> {
    let conn = Connect::open(Some("qemu:///system"))?;
    let _domain = Domain::create_xml(&conn, xml, 0)?;
    Ok(())
}
