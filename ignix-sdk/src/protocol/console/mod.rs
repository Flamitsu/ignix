mod simple_text_input;
mod simple_text_output;
pub use simple_text_input::{
    KeyData, KeyNotifyFunction, KeyShiftState, KeyState, KeyToggleState, SimpleTextInputProtocol,
    SimpleTextInputProtocolWrapper,
};
pub use simple_text_output::{
    SimpleTextOutputMode, SimpleTextOutputProtocol, SimpleTextOutputProtocolWrapper,
};
