use crate::IgnixError;
/// This function should remove the ignix installation in the current ESP partition.
pub fn remove_ignix_installation(_confirmation: bool) -> Result<(), IgnixError>{
    todo!("Remove the Ignix installation in the ESP and the NVRAM");
}
