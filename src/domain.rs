use crate::cli::CliArgs;
use crate::cli::FIRMWARE_TYPE_KEY;
use xmltree::{Element, XMLNode};

pub const LOADER_KEY: &str = "loader";
pub const LOADER_SECURE_KEY: &str = "loader_secure";

/// Build the libvirt domain XML dynamically
pub fn generate_domain_xml(args: &CliArgs) -> String {
    let mut domain = Element::new("domain");
    domain.attributes.insert("type".to_string(), "kvm".to_string());

    let name = Element::node("name").with_text(args.name.clone());
    domain.children.push(XMLNode::Element(name));
    
    let mut memory = Element::node("memory").with_text(args.memory.to_string());
    memory.attributes.insert("unit".to_string(), "MiB".to_string());
    domain.children.push(XMLNode::Element(memory));
    domain.children.push(XMLNode::Element(Element::node("vcpu").with_text(args.vcpus.to_string())));

    // OS Section
    let mut os = Element::new("os");
    if let Some(firmware_type) = args.boot.get(FIRMWARE_TYPE_KEY) {
        os = os.with_attr("type", firmware_type);
    }
    // Parse the value passed to --boot argument.
    if let Some(firmware_type) = args.boot.get(FIRMWARE_TYPE_KEY) {
        os = os.with_attr("type", firmware_type);
    }
    // Define the loader
    let mut loader = Element::node("loader");
    if let Some(loader_secure) = args.boot.get(LOADER_SECURE_KEY) {
        loader = loader.with_attr("secure", loader_secure);
    }
    if let Some(loader_path) = args.boot.get(LOADER_KEY) {
        loader = loader.with_text(loader_path.to_string());
    }
    os.children.push(XMLNode::Element(loader));
   
    os.children.push(XMLNode::Element(Element::node("type").with_text("hvm".to_string()).with_attr("arch", "x86_64")));

    let boot1 = Element::node("boot").with_attr("dev", "cdrom");
    let boot2 = Element::node("boot").with_attr("dev", "hd");

    os.children.push(XMLNode::Element(boot1));
    os.children.push(XMLNode::Element(boot2));
    domain.children.push(XMLNode::Element(os));

    // Devices Section
    let mut devices = Element::new("devices");

    // Primary Disk
    let mut disk = Element::new("disk");
    disk.attributes.insert("type".to_string(), "file".to_string());
    disk.attributes.insert("device".to_string(), "disk".to_string());

    let driver = Element::node("driver").with_attr("name", "qemu").with_attr("type", "qcow2");
    let source = Element::node("source").with_attr("file", &args.disk_path);
    let target = Element::node("target").with_attr("dev", "vda").with_attr("bus", "virtio");

    disk.children.push(XMLNode::Element(driver));
    disk.children.push(XMLNode::Element(source));
    disk.children.push(XMLNode::Element(target));
    devices.children.push(XMLNode::Element(disk));

    // Optional CD-ROM
    if let Some(cdrom_path) = &args.cdrom_path {
        let mut cdrom = Element::new("disk");
        cdrom.attributes.insert("type".to_string(), "file".to_string());
        cdrom.attributes.insert("device".to_string(), "cdrom".to_string());

        let driver = Element::node("driver").with_attr("name", "qemu").with_attr("type", "raw");
        let source = Element::node("source").with_attr("file", cdrom_path);
        let target = Element::node("target").with_attr("dev", "hdc").with_attr("bus", "ide");
        let readonly = Element::new("readonly");

        cdrom.children.push(XMLNode::Element(driver));
        cdrom.children.push(XMLNode::Element(source));
        cdrom.children.push(XMLNode::Element(target));
        cdrom.children.push(XMLNode::Element(readonly));

        devices.children.push(XMLNode::Element(cdrom));
    }

    // Console
    devices.children.push(XMLNode::Element(Element::node("console").with_attr("type", "pty")));

    domain.children.push(XMLNode::Element(devices));

    // Now serialize the whole domain
    let mut xml = Vec::new();
    domain.write(&mut xml).unwrap();

    String::from_utf8(xml).unwrap()
}

// Helper trait to make creating XML elements easier
trait XmlHelper {
    fn node(name: &str) -> Self;
    fn with_text(self, text: String) -> Self;
    fn with_attr(self, key: &str, value: &str) -> Self;
}

impl XmlHelper for Element {
    fn node(name: &str) -> Self {
        Element::new(name)
    }

    fn with_text(mut self, text: String) -> Self {
        self.children.push(XMLNode::Text(text));
        self
    }

    fn with_attr(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
}
