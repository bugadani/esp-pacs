#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - Configure slave addresses 0-1 of RTC I2C"]
    pub sar_slave_addr1: SAR_SLAVE_ADDR1,
    #[doc = "0x44 - Configure slave addresses 2-3 of RTC I2C"]
    pub sar_slave_addr2: SAR_SLAVE_ADDR2,
    #[doc = "0x48 - Configure slave addresses 4-5 of RTC I2C"]
    pub sar_slave_addr3: SAR_SLAVE_ADDR3,
    #[doc = "0x4c - Configure slave addresses 6-7 of RTC I2C"]
    pub sar_slave_addr4: SAR_SLAVE_ADDR4,
    _reserved4: [u8; 0x08],
    #[doc = "0x58 - Configure RTC I2C transmission"]
    pub sar_i2c_ctrl: SAR_I2C_CTRL,
    _reserved5: [u8; 0xcc],
    #[doc = "0x128 - Interrupt raw bit of ULP-RISCV"]
    pub sar_cocpu_int_raw: SAR_COCPU_INT_RAW,
    #[doc = "0x12c - Interrupt enable bit of ULP-RISCV"]
    pub sar_cocpu_int_ena: SAR_COCPU_INT_ENA,
    #[doc = "0x130 - Interrupt status bit of ULP-RISCV"]
    pub sar_cocpu_int_st: SAR_COCPU_INT_ST,
    #[doc = "0x134 - Interrupt clear bit of ULP-RISCV"]
    pub sar_cocpu_int_clr: SAR_COCPU_INT_CLR,
}
#[doc = "SAR_SLAVE_ADDR1 (rw) register accessor: Configure slave addresses 0-1 of RTC I2C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_slave_addr1`] module"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
#[doc = "Configure slave addresses 0-1 of RTC I2C"]
pub mod sar_slave_addr1;
#[doc = "SAR_SLAVE_ADDR2 (rw) register accessor: Configure slave addresses 2-3 of RTC I2C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_slave_addr2`] module"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
#[doc = "Configure slave addresses 2-3 of RTC I2C"]
pub mod sar_slave_addr2;
#[doc = "SAR_SLAVE_ADDR3 (rw) register accessor: Configure slave addresses 4-5 of RTC I2C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_slave_addr3`] module"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
#[doc = "Configure slave addresses 4-5 of RTC I2C"]
pub mod sar_slave_addr3;
#[doc = "SAR_SLAVE_ADDR4 (rw) register accessor: Configure slave addresses 6-7 of RTC I2C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_slave_addr4`] module"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
#[doc = "Configure slave addresses 6-7 of RTC I2C"]
pub mod sar_slave_addr4;
#[doc = "SAR_I2C_CTRL (rw) register accessor: Configure RTC I2C transmission\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_i2c_ctrl`] module"]
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
#[doc = "Configure RTC I2C transmission"]
pub mod sar_i2c_ctrl;
#[doc = "SAR_COCPU_INT_RAW (r) register accessor: Interrupt raw bit of ULP-RISCV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_cocpu_int_raw`] module"]
pub type SAR_COCPU_INT_RAW = crate::Reg<sar_cocpu_int_raw::SAR_COCPU_INT_RAW_SPEC>;
#[doc = "Interrupt raw bit of ULP-RISCV"]
pub mod sar_cocpu_int_raw;
#[doc = "SAR_COCPU_INT_ENA (rw) register accessor: Interrupt enable bit of ULP-RISCV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_cocpu_int_ena`] module"]
pub type SAR_COCPU_INT_ENA = crate::Reg<sar_cocpu_int_ena::SAR_COCPU_INT_ENA_SPEC>;
#[doc = "Interrupt enable bit of ULP-RISCV"]
pub mod sar_cocpu_int_ena;
#[doc = "SAR_COCPU_INT_ST (r) register accessor: Interrupt status bit of ULP-RISCV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_cocpu_int_st`] module"]
pub type SAR_COCPU_INT_ST = crate::Reg<sar_cocpu_int_st::SAR_COCPU_INT_ST_SPEC>;
#[doc = "Interrupt status bit of ULP-RISCV"]
pub mod sar_cocpu_int_st;
#[doc = "SAR_COCPU_INT_CLR (w) register accessor: Interrupt clear bit of ULP-RISCV\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_cocpu_int_clr`] module"]
pub type SAR_COCPU_INT_CLR = crate::Reg<sar_cocpu_int_clr::SAR_COCPU_INT_CLR_SPEC>;
#[doc = "Interrupt clear bit of ULP-RISCV"]
pub mod sar_cocpu_int_clr;
