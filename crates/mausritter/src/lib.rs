uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn add_with_one(left: u64, right: u64) -> u64 {
    mausritter_core::add(left, right)
}
