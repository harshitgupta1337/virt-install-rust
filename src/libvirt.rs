use crate::error::VirtInstallError;
//use virt::connect::Connect;

pub fn create_and_start_domain(_xml: &str) -> Result<(), VirtInstallError> {
    //let conn = Connect::open("qemu:///system")?;
    //let domain = conn.domain_define_xml(xml)?;
    //domain.create()?;
    Ok(())
}
