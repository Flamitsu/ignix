mod boolean;
mod errors;
mod event;
mod handler;
mod memory;
mod misc;
mod time;
pub use boolean::Boolean;

pub use event::{
    Event, EventGroup, EventNotifyFn, EventType, IgnixEvent, TimerDelay, Tpl, TplGuardian,
};

pub use memory::{
    AllocateType, DebugDisposition, MemoryAttributes, MemoryDescriptor, MemoryMap, MemoryType,
    PAGE_SIZE,
};

pub use errors::{IgnixError, Status};

pub use handler::{
    DevicePath, FixedHandleList, Handle, HandleBuffer, IgnixProtocol, IgnixProtocolNotification,
    InterfaceType, OpenProtocolAttributes, OpenProtocolInformation, OpenProtocolInformationEntry,
    ProtocolGuard, ProtocolsPerHandle, SearchType,
};

pub use misc::{
    Char16, DevicePathProtocol, Guid, IgnixImage, PhysicalAddress, Table, Uuid, VirtualAddress,
};

pub use time::{Time, TimeCapabilities, TimeStruct, WakeupTime};
