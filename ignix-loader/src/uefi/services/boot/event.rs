use crate::uefi::{init::SYSTEM_TABLE, table::boot::BootServicesWrapper, types::Tpl};
impl BootServicesWrapper{
    pub fn raise_tpl(&self, new_tpl: Tpl) -> Option<TplGuardian> {
        if let Some(function) = self.get_method(){
            return Some(TplGuardian { old_tlp: unsafe {(function.raise_tpl)(new_tpl)} });
        }
        None
    }

    pub fn restore_tpl(&self, old_tpl: Tpl){
        if let Some(function) = self.get_method(){
            unsafe {(function.restore_tpl)(old_tpl)}
        }
    }
}

/* NOTE FROM THE UEFI SPEC:
 * If NewTPL is below the current TPL level, then the system behaviour is indeterminate.
 * Executing TPLs ABOVE TPL_APPLICATION for longer periods of time may also result 
 * in unpredictable behaviour
 * ( I was wondering how to manage this, looked to uefi-rs code in uefi/src/boot.rs 
 * and it shows this same solution. Thank you guys. 
 * SDPX-License identifier: MIT OR Apache 2.0 )*/
pub struct TplGuardian{
    pub old_tlp: Tpl
}

impl TplGuardian{
    #[must_use]
    pub const fn get_old_tpl(&self) -> Tpl{
        self.old_tlp
    }
}

impl Drop for TplGuardian{
    fn drop(&mut self){
        SYSTEM_TABLE.get().unwrap().get_boot_services().unwrap().restore_tpl(self.old_tlp);
    }
}
