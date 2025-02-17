#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0xfc],
    #[doc = "0xfc - configure ulp"]
    pub rtc_ulp_cp_timer: RTC_ULP_CP_TIMER,
    #[doc = "0x100 - configure ulp"]
    pub rtc_ulp_cp_ctrl: RTC_ULP_CP_CTRL,
    #[doc = "0x104 - configure ulp-riscv"]
    pub rtc_cocpu_ctrl: RTC_COCPU_CTRL,
    _reserved3: [u8; 0x2c],
    #[doc = "0x134 - configure ulp sleep time"]
    pub rtc_ulp_cp_timer_1: RTC_ULP_CP_TIMER_1,
}
#[doc = "RTC_ULP_CP_TIMER (rw) register accessor: configure ulp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_ulp_cp_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_ulp_cp_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_ulp_cp_timer`] module"]
pub type RTC_ULP_CP_TIMER = crate::Reg<rtc_ulp_cp_timer::RTC_ULP_CP_TIMER_SPEC>;
#[doc = "configure ulp"]
pub mod rtc_ulp_cp_timer;
#[doc = "RTC_ULP_CP_CTRL (rw) register accessor: configure ulp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_ulp_cp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_ulp_cp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_ulp_cp_ctrl`] module"]
pub type RTC_ULP_CP_CTRL = crate::Reg<rtc_ulp_cp_ctrl::RTC_ULP_CP_CTRL_SPEC>;
#[doc = "configure ulp"]
pub mod rtc_ulp_cp_ctrl;
#[doc = "RTC_COCPU_CTRL (rw) register accessor: configure ulp-riscv\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cocpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cocpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_cocpu_ctrl`] module"]
pub type RTC_COCPU_CTRL = crate::Reg<rtc_cocpu_ctrl::RTC_COCPU_CTRL_SPEC>;
#[doc = "configure ulp-riscv"]
pub mod rtc_cocpu_ctrl;
#[doc = "RTC_ULP_CP_TIMER_1 (rw) register accessor: configure ulp sleep time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_ulp_cp_timer_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_ulp_cp_timer_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_ulp_cp_timer_1`] module"]
pub type RTC_ULP_CP_TIMER_1 = crate::Reg<rtc_ulp_cp_timer_1::RTC_ULP_CP_TIMER_1_SPEC>;
#[doc = "configure ulp sleep time"]
pub mod rtc_ulp_cp_timer_1;
