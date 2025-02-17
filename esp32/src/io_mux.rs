#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub pin_ctrl: PIN_CTRL,
    #[doc = "0x04 - "]
    pub gpio36: GPIO36,
    #[doc = "0x08 - "]
    pub gpio37: GPIO37,
    #[doc = "0x0c - "]
    pub gpio38: GPIO38,
    #[doc = "0x10 - "]
    pub gpio39: GPIO39,
    #[doc = "0x14 - "]
    pub gpio34: GPIO34,
    #[doc = "0x18 - "]
    pub gpio35: GPIO35,
    #[doc = "0x1c - "]
    pub gpio32: GPIO32,
    #[doc = "0x20 - "]
    pub gpio33: GPIO33,
    #[doc = "0x24 - "]
    pub gpio25: GPIO25,
    #[doc = "0x28 - "]
    pub gpio26: GPIO26,
    #[doc = "0x2c - "]
    pub gpio27: GPIO27,
    #[doc = "0x30 - "]
    pub gpio14: GPIO14,
    #[doc = "0x34 - "]
    pub gpio12: GPIO12,
    #[doc = "0x38 - "]
    pub gpio13: GPIO13,
    #[doc = "0x3c - "]
    pub gpio15: GPIO15,
    #[doc = "0x40 - "]
    pub gpio2: GPIO2,
    #[doc = "0x44 - "]
    pub gpio0: GPIO0,
    #[doc = "0x48 - "]
    pub gpio4: GPIO4,
    #[doc = "0x4c - "]
    pub gpio16: GPIO16,
    #[doc = "0x50 - "]
    pub gpio17: GPIO17,
    #[doc = "0x54 - "]
    pub gpio9: GPIO9,
    #[doc = "0x58 - "]
    pub gpio10: GPIO10,
    #[doc = "0x5c - "]
    pub gpio11: GPIO11,
    #[doc = "0x60 - "]
    pub gpio6: GPIO6,
    #[doc = "0x64 - "]
    pub gpio7: GPIO7,
    #[doc = "0x68 - "]
    pub gpio8: GPIO8,
    #[doc = "0x6c - "]
    pub gpio5: GPIO5,
    #[doc = "0x70 - "]
    pub gpio18: GPIO18,
    #[doc = "0x74 - "]
    pub gpio19: GPIO19,
    #[doc = "0x78 - "]
    pub gpio20: GPIO20,
    #[doc = "0x7c - "]
    pub gpio21: GPIO21,
    #[doc = "0x80 - "]
    pub gpio22: GPIO22,
    #[doc = "0x84 - "]
    pub gpio3: GPIO3,
    #[doc = "0x88 - "]
    pub gpio1: GPIO1,
    #[doc = "0x8c - "]
    pub gpio23: GPIO23,
    #[doc = "0x90 - "]
    pub gpio24: GPIO24,
}
#[doc = "PIN_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin_ctrl`] module"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = ""]
pub mod pin_ctrl;
#[doc = "GPIO36 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio36`] module"]
pub type GPIO36 = crate::Reg<gpio36::GPIO36_SPEC>;
#[doc = ""]
pub mod gpio36;
#[doc = "GPIO37 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio37`] module"]
pub type GPIO37 = crate::Reg<gpio37::GPIO37_SPEC>;
#[doc = ""]
pub mod gpio37;
#[doc = "GPIO38 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio38`] module"]
pub type GPIO38 = crate::Reg<gpio38::GPIO38_SPEC>;
#[doc = ""]
pub mod gpio38;
#[doc = "GPIO39 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio39`] module"]
pub type GPIO39 = crate::Reg<gpio39::GPIO39_SPEC>;
#[doc = ""]
pub mod gpio39;
#[doc = "GPIO34 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio34`] module"]
pub type GPIO34 = crate::Reg<gpio34::GPIO34_SPEC>;
#[doc = ""]
pub mod gpio34;
#[doc = "GPIO35 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio35`] module"]
pub type GPIO35 = crate::Reg<gpio35::GPIO35_SPEC>;
#[doc = ""]
pub mod gpio35;
#[doc = "GPIO32 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio32`] module"]
pub type GPIO32 = crate::Reg<gpio32::GPIO32_SPEC>;
#[doc = ""]
pub mod gpio32;
#[doc = "GPIO33 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio33`] module"]
pub type GPIO33 = crate::Reg<gpio33::GPIO33_SPEC>;
#[doc = ""]
pub mod gpio33;
#[doc = "GPIO25 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio25`] module"]
pub type GPIO25 = crate::Reg<gpio25::GPIO25_SPEC>;
#[doc = ""]
pub mod gpio25;
#[doc = "GPIO26 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio26`] module"]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = ""]
pub mod gpio26;
#[doc = "GPIO27 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio27`] module"]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = ""]
pub mod gpio27;
#[doc = "GPIO14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio14`] module"]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = ""]
pub mod gpio14;
#[doc = "GPIO12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio12`] module"]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = ""]
pub mod gpio12;
#[doc = "GPIO13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio13`] module"]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = ""]
pub mod gpio13;
#[doc = "GPIO15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio15`] module"]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = ""]
pub mod gpio15;
#[doc = "GPIO2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio2`] module"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = ""]
pub mod gpio2;
#[doc = "GPIO0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio0`] module"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = ""]
pub mod gpio0;
#[doc = "GPIO4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio4`] module"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = ""]
pub mod gpio4;
#[doc = "GPIO16 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio16`] module"]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = ""]
pub mod gpio16;
#[doc = "GPIO17 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio17`] module"]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = ""]
pub mod gpio17;
#[doc = "GPIO9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio9`] module"]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = ""]
pub mod gpio9;
#[doc = "GPIO10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio10`] module"]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = ""]
pub mod gpio10;
#[doc = "GPIO11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio11`] module"]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = ""]
pub mod gpio11;
#[doc = "GPIO6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio6`] module"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = ""]
pub mod gpio6;
#[doc = "GPIO7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio7`] module"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = ""]
pub mod gpio7;
#[doc = "GPIO8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio8`] module"]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = ""]
pub mod gpio8;
#[doc = "GPIO5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio5`] module"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = ""]
pub mod gpio5;
#[doc = "GPIO18 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio18`] module"]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = ""]
pub mod gpio18;
#[doc = "GPIO19 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio19`] module"]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = ""]
pub mod gpio19;
#[doc = "GPIO20 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio20`] module"]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = ""]
pub mod gpio20;
#[doc = "GPIO21 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio21`] module"]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = ""]
pub mod gpio21;
#[doc = "GPIO22 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio22`] module"]
pub type GPIO22 = crate::Reg<gpio22::GPIO22_SPEC>;
#[doc = ""]
pub mod gpio22;
#[doc = "GPIO3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio3`] module"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = ""]
pub mod gpio3;
#[doc = "GPIO1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio1`] module"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = ""]
pub mod gpio1;
#[doc = "GPIO23 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio23`] module"]
pub type GPIO23 = crate::Reg<gpio23::GPIO23_SPEC>;
#[doc = ""]
pub mod gpio23;
#[doc = "GPIO24 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio24`] module"]
pub type GPIO24 = crate::Reg<gpio24::GPIO24_SPEC>;
#[doc = ""]
pub mod gpio24;
