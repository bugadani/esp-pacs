#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO bit select register"]
    pub bt_select: BT_SELECT,
    #[doc = "0x04 - GPIO output register for GPIO0-31"]
    pub out: OUT,
    #[doc = "0x08 - GPIO output set register for GPIO0-31"]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x0c - GPIO output clear register for GPIO0-31"]
    pub out_w1tc: OUT_W1TC,
    #[doc = "0x10 - GPIO output register for GPIO32-34"]
    pub out1: OUT1,
    #[doc = "0x14 - GPIO output set register for GPIO32-34"]
    pub out1_w1ts: OUT1_W1TS,
    #[doc = "0x18 - GPIO output clear register for GPIO32-34"]
    pub out1_w1tc: OUT1_W1TC,
    #[doc = "0x1c - GPIO sdio select register"]
    pub sdio_select: SDIO_SELECT,
    #[doc = "0x20 - GPIO output enable register for GPIO0-31"]
    pub enable: ENABLE,
    #[doc = "0x24 - GPIO output enable set register for GPIO0-31"]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x28 - GPIO output enable clear register for GPIO0-31"]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x2c - GPIO output enable register for GPIO32-34"]
    pub enable1: ENABLE1,
    #[doc = "0x30 - GPIO output enable set register for GPIO32-34"]
    pub enable1_w1ts: ENABLE1_W1TS,
    #[doc = "0x34 - GPIO output enable clear register for GPIO32-34"]
    pub enable1_w1tc: ENABLE1_W1TC,
    #[doc = "0x38 - pad strapping register"]
    pub strap: STRAP,
    #[doc = "0x3c - GPIO input register for GPIO0-31"]
    pub in_: IN,
    #[doc = "0x40 - GPIO input register for GPIO32-34"]
    pub in1: IN1,
    #[doc = "0x44 - GPIO interrupt status register for GPIO0-31"]
    pub status: STATUS,
    #[doc = "0x48 - GPIO interrupt status set register for GPIO0-31"]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x4c - GPIO interrupt status clear register for GPIO0-31"]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x50 - GPIO interrupt status register for GPIO32-34"]
    pub status1: STATUS1,
    #[doc = "0x54 - GPIO interrupt status set register for GPIO32-34"]
    pub status1_w1ts: STATUS1_W1TS,
    #[doc = "0x58 - GPIO interrupt status clear register for GPIO32-34"]
    pub status1_w1tc: STATUS1_W1TC,
    #[doc = "0x5c - GPIO PRO_CPU interrupt status register for GPIO0-31"]
    pub pcpu_int: PCPU_INT,
    #[doc = "0x60 - GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
    pub pcpu_nmi_int: PCPU_NMI_INT,
    #[doc = "0x64 - GPIO CPUSDIO interrupt status register for GPIO0-31"]
    pub cpusdio_int: CPUSDIO_INT,
    #[doc = "0x68 - GPIO PRO_CPU interrupt status register for GPIO32-34"]
    pub pcpu_int1: PCPU_INT1,
    #[doc = "0x6c - GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-34"]
    pub pcpu_nmi_int1: PCPU_NMI_INT1,
    #[doc = "0x70 - GPIO CPUSDIO interrupt status register for GPIO32-34"]
    pub cpusdio_int1: CPUSDIO_INT1,
    #[doc = "0x74..0x100 - GPIO pin configuration register"]
    pub pin: [PIN; 35],
    _reserved30: [u8; 0x4c],
    #[doc = "0x14c - GPIO interrupt source register for GPIO0-31"]
    pub status_next: STATUS_NEXT,
    #[doc = "0x150 - GPIO interrupt source register for GPIO32-34"]
    pub status_next1: STATUS_NEXT1,
    #[doc = "0x154..0x354 - GPIO input function configuration register"]
    pub func_in_sel_cfg: [FUNC_IN_SEL_CFG; 128],
    _reserved33: [u8; 0x0200],
    #[doc = "0x554..0x5cc - GPIO output function select register"]
    pub func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 30],
    _reserved34: [u8; 0x60],
    #[doc = "0x62c - GPIO clock gate register"]
    pub clock_gate: CLOCK_GATE,
    _reserved35: [u8; 0xcc],
    #[doc = "0x6fc - GPIO version register"]
    pub date: DATE,
}
#[doc = "BT_SELECT (rw) register accessor: GPIO bit select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_select`] module"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = "GPIO bit select register"]
pub mod bt_select;
#[doc = "OUT (rw) register accessor: GPIO output register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register for GPIO0-31"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: GPIO output set register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO0-31"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: GPIO output clear register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO0-31"]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: GPIO output register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "GPIO output register for GPIO32-34"]
pub mod out1;
#[doc = "OUT1_W1TS (w) register accessor: GPIO output set register for GPIO32-34\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO32-34"]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (w) register accessor: GPIO output clear register for GPIO32-34\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO32-34"]
pub mod out1_w1tc;
#[doc = "SDIO_SELECT (rw) register accessor: GPIO sdio select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_select`] module"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = "GPIO sdio select register"]
pub mod sdio_select;
#[doc = "ENABLE (rw) register accessor: GPIO output enable register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register for GPIO0-31"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: GPIO output enable set register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO0-31"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: GPIO output enable clear register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO0-31"]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: GPIO output enable register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = "GPIO output enable register for GPIO32-34"]
pub mod enable1;
#[doc = "ENABLE1_W1TS (w) register accessor: GPIO output enable set register for GPIO32-34\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO32-34"]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (w) register accessor: GPIO output enable clear register for GPIO32-34\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO32-34"]
pub mod enable1_w1tc;
#[doc = "STRAP (r) register accessor: pad strapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "pad strapping register"]
pub mod strap;
#[doc = "IN (r) register accessor: GPIO input register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register for GPIO0-31"]
pub mod in_;
#[doc = "IN1 (r) register accessor: GPIO input register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = "GPIO input register for GPIO32-34"]
pub mod in1;
#[doc = "STATUS (rw) register accessor: GPIO interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register for GPIO0-31"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: GPIO interrupt status set register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO0-31"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO0-31"]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: GPIO interrupt status register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "GPIO interrupt status register for GPIO32-34"]
pub mod status1;
#[doc = "STATUS1_W1TS (w) register accessor: GPIO interrupt status set register for GPIO32-34\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO32-34"]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO32-34\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO32-34"]
pub mod status1_w1tc;
#[doc = "PCPU_INT (r) register accessor: GPIO PRO_CPU interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_int`] module"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register for GPIO0-31"]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT (r) register accessor: GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_nmi_int`] module"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT (r) register accessor: GPIO CPUSDIO interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpusdio_int`] module"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register for GPIO0-31"]
pub mod cpusdio_int;
#[doc = "PCPU_INT1 (r) register accessor: GPIO PRO_CPU interrupt status register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_int1`] module"]
pub type PCPU_INT1 = crate::Reg<pcpu_int1::PCPU_INT1_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register for GPIO32-34"]
pub mod pcpu_int1;
#[doc = "PCPU_NMI_INT1 (r) register accessor: GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_nmi_int1`] module"]
pub type PCPU_NMI_INT1 = crate::Reg<pcpu_nmi_int1::PCPU_NMI_INT1_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-34"]
pub mod pcpu_nmi_int1;
#[doc = "CPUSDIO_INT1 (r) register accessor: GPIO CPUSDIO interrupt status register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpusdio_int1`] module"]
pub type CPUSDIO_INT1 = crate::Reg<cpusdio_int1::CPUSDIO_INT1_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register for GPIO32-34"]
pub mod cpusdio_int1;
#[doc = "PIN (rw) register accessor: GPIO pin configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin;
#[doc = "STATUS_NEXT (r) register accessor: GPIO interrupt source register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register for GPIO0-31"]
pub mod status_next;
#[doc = "STATUS_NEXT1 (r) register accessor: GPIO interrupt source register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_next1`] module"]
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
#[doc = "GPIO interrupt source register for GPIO32-34"]
pub mod status_next1;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`func_in_sel_cfg`] module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: GPIO output function select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func_out_sel_cfg;
#[doc = "CLOCK_GATE (rw) register accessor: GPIO clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "GPIO version register"]
pub mod date;
