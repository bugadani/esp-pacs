#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub clk_en: CLK_EN,
    #[doc = "0x04 - need_des"]
    pub reset_en: RESET_EN,
    #[doc = "0x08 - need_des"]
    pub rng_data: RNG_DATA,
    #[doc = "0x0c - need_des"]
    pub cpu: CPU,
    #[doc = "0x10 - need_des"]
    pub bus_timeout: BUS_TIMEOUT,
    #[doc = "0x14 - need_des"]
    pub bus_timeout_addr: BUS_TIMEOUT_ADDR,
    #[doc = "0x18 - need_des"]
    pub bus_timeout_uid: BUS_TIMEOUT_UID,
    #[doc = "0x1c - need_des"]
    pub mem_ctrl: MEM_CTRL,
    #[doc = "0x20 - need_des"]
    pub interrupt_source: INTERRUPT_SOURCE,
    _reserved9: [u8; 0x03d8],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "CLK_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "need_des"]
pub mod clk_en;
#[doc = "RESET_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reset_en`] module"]
pub type RESET_EN = crate::Reg<reset_en::RESET_EN_SPEC>;
#[doc = "need_des"]
pub mod reset_en;
#[doc = "RNG_DATA (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rng_data`] module"]
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
#[doc = "need_des"]
pub mod rng_data;
#[doc = "CPU (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu`] module"]
pub type CPU = crate::Reg<cpu::CPU_SPEC>;
#[doc = "need_des"]
pub mod cpu;
#[doc = "BUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bus_timeout`] module"]
pub type BUS_TIMEOUT = crate::Reg<bus_timeout::BUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout;
#[doc = "BUS_TIMEOUT_ADDR (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bus_timeout_addr`] module"]
pub type BUS_TIMEOUT_ADDR = crate::Reg<bus_timeout_addr::BUS_TIMEOUT_ADDR_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout_addr;
#[doc = "BUS_TIMEOUT_UID (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout_uid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bus_timeout_uid`] module"]
pub type BUS_TIMEOUT_UID = crate::Reg<bus_timeout_uid::BUS_TIMEOUT_UID_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout_uid;
#[doc = "MEM_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_ctrl`] module"]
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
#[doc = "need_des"]
pub mod mem_ctrl;
#[doc = "INTERRUPT_SOURCE (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_source::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`interrupt_source`] module"]
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
#[doc = "need_des"]
pub mod interrupt_source;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
