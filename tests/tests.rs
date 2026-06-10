use xanterella::get::*;

#[test]
fn test_sshstring() {
    assert_eq!(get_sshstring(String::from("192.125.142.2")), "root@192.125.142.2")
}

#[test]
fn test_drivenames() {
    assert_eq!(get_drives_name(&String::from("nvme0n1"), 1), "nvme0n1p1");
    assert_eq!(get_drives_name(&String::from("sda"), 2), "sda2");
}
