// SPDX-License-Identifier: GPL-3.0-onl
use crate::types::Boolean;

#[repr(C)]
pub struct Time {
    pub year: u16,  // 1900 - 9999
    pub month: u8,  // 1 - 12
    pub day: u8,    // 1 - 31
    pub hour: u8,   // 0 - 23
    pub minute: u8, // 0 - 59
    pub second: u8, // 0 - 59
    pad1: u8,
    pub nanosecond: u32, // 0 - 999,999,999
    /*
     * The time’s offset in minutes from UTC. If the value is EFI_UNSPECIFIED_TIMEZONE, then the
     * time is interpreted as a local time. The TimeZone is the number of minutes that the local
     * time is relative to UTC. To calculate the TimeZone value, follow this equation: Localtime
     * = UTC - TimeZone.*/
    pub timezone: i16, // -1440 to 1440 or 2047
    /*
     * A bitmask containing the daylight savings time information for the time.
     * The EFI_TIME_ADJUST_DAYLIGHT bit indicates if the time is affected by daylight savings time
     * or not. This value does not indicate that the time has been adjusted for daylight savings
     * time. It indicates only that it should be adjusted when the EFI_TIME enters daylight savings
     * time. If EFI_TIME_IN_DAYLIGHT is set, the time has been adjusted for daylight savings time.
     *
     * All other bits must be zero.
     *
     * When entering daylight saving time, if the time is affected, but hasn’t been adjusted (DST =
     * 1), use the new calculation:
     * 1. The date/time should be increased by the appropriate amount.
     * 2. The TimeZone should be decreased by the appropriate amount (EX: +480 changes to +420 when
     * moving from PST to PDT).
     * 3. The Daylight value changes to 3.
     *
     * When exiting daylight saving time, if the time is affected and has been adjusted (DST = 3),
     * use the new calculation:
     * 1. The date/time should be decreased by the appropriate amount.
     * 2. The TimeZone should be increased by the appropriate amount.
     * 3. The Daylight value changes to 1.*/
    pub daylight: u8,
    pad2: u8,
}

/* Bit Definitions for EFI_TIME.Daylight.
 * EFI_TIME_ADJUST_DAYLIGHT 0x01
 * EFI_TIME_IN_DAYLIGHT 0x02
 *
 * Value Definition for EFI_TIME.TimeZone.
 * EFI_UNSPECIFIED_TIMEZONE 0x07FF
*/
impl Time {
    pub fn is_adjust_daylight(&self) -> bool {
        self.daylight == 0x01
    }
    pub fn is_in_daylight(&self) -> bool {
        self.daylight == 0x02
    }
    pub fn is_unespecified_timezone(&self) -> bool {
        self.timezone == 0x07FF
    }
}
#[repr(C)]
pub struct TimeCapabilities {
    /* Provides the reporting resolution of the RTC per second.
     * For normal RTCs devices this value should be 1Hz or 1 to indicate that it reports
     * the time resolution of 1 second*/
    pub resolution: u32,
    /*For a clock with an accuracy of 50 parts per million, the value will be 50,000,000*/
    pub accuracy: u32,
    /* true indicates that the device's time below the resolution reporting level. false indicates
     * that the state below the Resolution level of the device is not cleared when the time is set.
     */
    pub sets_to_zero: bool,
}

#[repr(C)]
pub struct TimeStruct {
    pub time: Time,
    pub time_capabilities: TimeCapabilities,
}

pub struct WakeupTime {
    pub enabled: Boolean,
    pub pending: Boolean,
    pub time: Time,
}
