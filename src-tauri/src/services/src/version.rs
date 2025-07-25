use utils::version::Version;

pub fn version_is_less(version: &str, target: &str) -> bool {
    let version = Version::new(version);
    let target = Version::new(target);
    version < target
}

pub fn version_is_equal(version: &str, target: &str) -> bool {
    let version = Version::new(version);
    let target = Version::new(target);
    version == target
}

pub fn version_is_greater(version: &str, target: &str) -> bool {
    let version = Version::new(version);
    let target = Version::new(target);
    version > target
}

pub fn version_is_greater_equal(version: &str, target: &str) -> bool {
    let version = Version::new(version);
    let target = Version::new(target);
    version >= target
}

pub fn version_is_less_equal(version: &str, target: &str) -> bool {
    let version = Version::new(version);
    let target = Version::new(target);
    version < target
}

pub fn version_compare(version: &str, target: &str) -> i32 {
    let version = Version::new(version);
    let target = Version::new(target);
    version.compare(&target)
}
