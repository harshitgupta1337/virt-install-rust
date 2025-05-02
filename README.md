# virt-install-rust

## Current status

Currently, the tool can be used to create a simple VM based on an existing OS Disk image.
The following command can be used for that.

```bash
# Download the disk image
wget -c https://cloud-images.ubuntu.com/jammy/20250308/jammy-server-cloudimg-amd64.img

# Build the virt-install-rs binary
cargo build

# Create a VM from the above disk image
target/debug/virt-install-rs \
    --name testvm \
    --memory 1024 \
    --vcpus 2 \
    --boot uefi,loader_secure=yes,loader=/usr/share/OVMF/OVMF_CODE_4M.fd \
    --disk-path $(realpath jammy-server-cloudimg-amd64.img)
```
