// vk2dfd/src/test.rs

use crate::table::VK_TO_DFD;

#[test]
fn sizes_are_sensible() {
    for (_, value) in VK_TO_DFD.entries() {
        assert_eq!(value[0], value.len() as u32 * 4);
    }
}

#[test]
fn lookups_work() {
    for (format, value) in VK_TO_DFD.entries() {
        assert_eq!(*value, crate::vk2dfd(*format).unwrap());
    }
}
