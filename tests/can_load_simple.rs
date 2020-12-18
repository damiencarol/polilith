use std::fs::File;
use tar::Archive;
#[path = "../src/docker.rs"] mod docker;

#[test]
fn can_load_simple_image() {
    let mut ar = Archive::new(File::open("busybox-1.32.0.tar").unwrap());
    let manifest = docker::get_manifest(&mut ar);
    assert_eq!("219ee5171f8006d1462fa76c12b9b01ab672dbc8b283f186841bf2c3ca8e3c93.json", manifest[0].config)
}
