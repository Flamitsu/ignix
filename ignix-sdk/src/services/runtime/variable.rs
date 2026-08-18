// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    table::get_runtime_services,
    types::{
        Guid, IgnixError, NextVariableName, NonVolatileRamStatus, Status, Variable,
        VariableAttributes,
    },
};
use core::{
    ffi::c_void,
    mem::zeroed,
    ptr::{null, null_mut},
};
/// Returns the value of a variable.
///
/// RETURN CODES:
/// EFI_NOT_FOUND The variable was not found.
/// EFI_BUFFER_TOO_SMALL The DataSize is too small for the result. DataSize has been updated
/// with the size needed to complete the request. If Attributes is not NULL, then the attributes
/// bitmask for the variable has been stored to the memory location pointed-to by Attributes.
/// EFI_INVALID_PARAMETER VariableName is NULL.
/// EFI_INVALID_PARAMETER VendorGuid is NULL.
/// EFI_INVALID_PARAMETER DataSize is NULL.
/// EFI_INVALID_PARAMETER The DataSize is not too small and Data is NULL.
/// EFI_DEVICE_ERROR The variable could not be retrieved due to a hardware error.
/// EFI_SECURITY_VIOLATION The variable could not be retrieved due to an authentication failure.
/// EFI_UNSUPPORTED After ExitBootServices() has been called, this return code may be returned
/// if no variable storage is supported. The platform should describe this runtime service as
/// unsupported at runtime via an EFI_RT_PROPERTIES_TABLE configuration table.
pub fn get_variable<'a, const BUFFER_SIZE: usize>(
    variable_name: &'a [u16],
    vendor_guid: &Guid,
) -> Result<Variable<'a, BUFFER_SIZE>, IgnixError> {
    let mut attr: VariableAttributes = unsafe { zeroed() };
    let mut data_size: usize = BUFFER_SIZE;
    let mut data = [0u8; BUFFER_SIZE];
    let status_one = unsafe {
        (get_runtime_services().get_variable)(
            variable_name.as_ptr(),
            vendor_guid,
            &mut attr,
            &mut data_size,
            data.as_mut_ptr() as *mut c_void,
        )
    };

    let status = unsafe {
        (get_runtime_services().get_variable)(
            variable_name.as_ptr(),
            vendor_guid,
            &mut attr,
            &mut data_size,
            data.as_mut_ptr() as *mut c_void,
        )
    };
    if status.is_error() {
        Err(status.context("get_variable"))?
    }
    Ok(Variable {
        variable_name,
        vendor_guid,
        attr,
        data_size,
        data,
    })
}
/// Enumerates the current variable names.
///
/// RETURN CODES:
/// EFI_NOT_FOUND The next variable was not found.
/// EFI_BUFFER_TOO_SMALL The VariableNameSize is too small for the result. VariableNameSize has been updated with the size needed to complete the request.
/// EFI_INVALID_PARAMETER VariableNameSize is NULL.
/// EFI_INVALID_PARAMETER VariableName is NULL.
/// EFI_INVALID_PARAMETER VendorGuid is NULL.
/// EFI_INVALID_PARAMETER The input values of VariableName and VendorGuid are not a name and GUID of an existing variable.EFI_INVALID_PARAMETER Null-terminator is not found in the first VariableNameSize bytes of the input VariableName buffer.
/// EFI_DEVICE_ERROR The variable name could not be retrieved due to a hardware error.
/// EFI_UNSUPPORTED After ExitBootServices() has been called, this return code may be returned
/// if no variable storage is supported. The platform should describe this runtime
/// service as unsupported at runtime via an EFI_RT_PROPERTIES_TABLE configuration table.
pub fn get_next_variable_name<const N: usize>() -> Result<NextVariableName<N>, IgnixError> {
    let variable_name_size = N;
    let mut variable_name = [0u16; N];
    let mut vendor_guid = Guid::new(0, 0, 0, [0u8; 8]);
    let status = unsafe {
        (get_runtime_services().get_next_variable_name)(
            variable_name_size as *mut usize,
            variable_name.as_mut_ptr(),
            &mut vendor_guid as *mut Guid,
        )
    };

    if status.is_error() {
        Err(status.context("get_next_variable_name"))?
    }
    Ok(NextVariableName {
        variable_name_size,
        variable_name,
        vendor_guid: &vendor_guid as *const Guid,
    })
}
/// EFI_INVALID_PARAMETER An invalid combination of attribute bits, name, and GUID was supplied, or the DataSize exceeds the maximum allowed.
/// EFI_INVALID_PARAMETER VariableName is an empty string.
/// EFI_OUT_OF_RESOURCES Not enough storage is available to hold the variable and its data.
/// EFI_DEVICE_ERROR The variable could not be saved due to a hardware failure.
/// EFI_WRITE_PROTECTED The variable in question is read-only.
/// EFI_WRITE_PROTECTED The variable in question cannot be deleted.
/// EFI_SECURITY_VIOLATION The variable could not be written due to EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS or EFI_VARI ABLE_TIME_BASED_AUTHENTICATED_WRITE_ACESS being set, but the payload does NOT pass the validation check carried out by the firmware.
/// EFI_NOT_FOUND The variable trying to be updated or deleted was not found.
/// EFI_UNSUPPORTED This call is not supported by this platform at the time the call is made. The
/// platform should describe this runtime service as unsupported at runtime via
/// an EFI_RT_PROPERTIES_TABLE configuration table.
pub fn set_variable<const N: usize>(variable: &Variable<'_, N>) -> Result<(), IgnixError> {
    let status = unsafe {
        (get_runtime_services().set_variable)(
            variable.variable_name.as_ptr(),
            variable.vendor_guid,
            variable.attr,
            variable.data_size,
            variable.data.as_ptr() as *const c_void,
        )
    };
    if status.is_error() {
        Err(status.context("set_variable"))?
    }
    Ok(())
}

/// Returns information about the EFI variables.
///
/// RETURN CODES:
/// EFI_INVALID_PARAMETER An invalid combination of attribute bits was supplied
/// EFI_UNSUPPORTED The attribute is not supported on this platform, and the
/// MaximumVariableStorageSize,
/// RemainingVariableStorageSize,
/// MaximumVariableSize are undefined.
pub fn query_variable_info(attr: VariableAttributes) -> Result<NonVolatileRamStatus, IgnixError> {
    let mut maximum_variable_storage_size: u64 = 0;
    let mut remaining_variable_storage_size: u64 = 0;
    let mut maximum_variable_size: u64 = 0;
    let status = unsafe {
        (get_runtime_services().query_variable_info)(
            attr,
            &mut maximum_variable_storage_size,
            &mut remaining_variable_storage_size,
            &mut maximum_variable_size,
        )
    };
    Ok(NonVolatileRamStatus {
        attr,
        maximum_variable_storage_size,
        maximum_variable_size,
        remaining_variable_storage_size,
    })
}
