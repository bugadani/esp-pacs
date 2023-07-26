#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - The read and write data register for CHANNEL%s by apb fifo access."]
    pub chdata: [CHDATA; 4],
    #[doc = "0x10..0x18 - Channel %s configure register 0"]
    pub ch_tx_conf0: [CH_TX_CONF0; 2],
    #[doc = "0x18 - Channel %s configure register 0"]
    pub ch2_rx_conf0: CH_RX_CONF0,
    #[doc = "0x1c - Channel %s configure register 1"]
    pub ch2_rx_conf1: CH_RX_CONF1,
    #[doc = "0x20 - Channel %s configure register 0"]
    pub ch3_rx_conf0: CH_RX_CONF0,
    #[doc = "0x24 - Channel %s configure register 1"]
    pub ch3_rx_conf1: CH_RX_CONF1,
    #[doc = "0x28..0x30 - Channel %s status register"]
    pub ch_tx_status: [CH_TX_STATUS; 2],
    #[doc = "0x30..0x38 - Channel %s status register"]
    pub ch_rx_status: [CH_RX_STATUS; 2],
    #[doc = "0x38 - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0x3c - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x40 - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x44 - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x48..0x50 - Channel %s duty cycle configuration register"]
    pub chcarrier_duty: [CHCARRIER_DUTY; 2],
    #[doc = "0x50..0x58 - Channel %s carrier remove register"]
    pub ch_rx_carrier_rm: [CH_RX_CARRIER_RM; 2],
    #[doc = "0x58..0x60 - Channel %s Tx event configuration register"]
    pub ch_tx_lim: [CH_TX_LIM; 2],
    #[doc = "0x60..0x68 - Channel %s Rx event configuration register"]
    pub ch_rx_lim: [CH_RX_LIM; 2],
    #[doc = "0x68 - RMT apb configuration register"]
    pub sys_conf: SYS_CONF,
    #[doc = "0x6c - RMT TX synchronous register"]
    pub tx_sim: TX_SIM,
    #[doc = "0x70 - RMT clock divider reset register"]
    pub ref_cnt_rst: REF_CNT_RST,
    _reserved19: [u8; 0x58],
    #[doc = "0xcc - RMT version register"]
    pub date: DATE,
}
#[doc = "CHDATA (rw) register accessor: an alias for `Reg<CHDATA_SPEC>`"]
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
#[doc = "The read and write data register for CHANNEL%s by apb fifo access."]
pub mod chdata;
#[doc = "CH_TX_CONF0 (rw) register accessor: an alias for `Reg<CH_TX_CONF0_SPEC>`"]
pub type CH_TX_CONF0 = crate::Reg<ch_tx_conf0::CH_TX_CONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod ch_tx_conf0;
#[doc = "CH_RX_CONF0 (rw) register accessor: an alias for `Reg<CH_RX_CONF0_SPEC>`"]
pub type CH_RX_CONF0 = crate::Reg<ch_rx_conf0::CH_RX_CONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod ch_rx_conf0;
#[doc = "CH_RX_CONF1 (rw) register accessor: an alias for `Reg<CH_RX_CONF1_SPEC>`"]
pub type CH_RX_CONF1 = crate::Reg<ch_rx_conf1::CH_RX_CONF1_SPEC>;
#[doc = "Channel %s configure register 1"]
pub mod ch_rx_conf1;
#[doc = "CH_TX_STATUS (r) register accessor: an alias for `Reg<CH_TX_STATUS_SPEC>`"]
pub type CH_TX_STATUS = crate::Reg<ch_tx_status::CH_TX_STATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod ch_tx_status;
#[doc = "CH_RX_STATUS (r) register accessor: an alias for `Reg<CH_RX_STATUS_SPEC>`"]
pub type CH_RX_STATUS = crate::Reg<ch_rx_status::CH_RX_STATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod ch_rx_status;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CHCARRIER_DUTY (rw) register accessor: an alias for `Reg<CHCARRIER_DUTY_SPEC>`"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = "Channel %s duty cycle configuration register"]
pub mod chcarrier_duty;
#[doc = "CH_RX_CARRIER_RM (rw) register accessor: an alias for `Reg<CH_RX_CARRIER_RM_SPEC>`"]
pub type CH_RX_CARRIER_RM = crate::Reg<ch_rx_carrier_rm::CH_RX_CARRIER_RM_SPEC>;
#[doc = "Channel %s carrier remove register"]
pub mod ch_rx_carrier_rm;
#[doc = "CH_TX_LIM (rw) register accessor: an alias for `Reg<CH_TX_LIM_SPEC>`"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = "Channel %s Tx event configuration register"]
pub mod ch_tx_lim;
#[doc = "CH_RX_LIM (rw) register accessor: an alias for `Reg<CH_RX_LIM_SPEC>`"]
pub type CH_RX_LIM = crate::Reg<ch_rx_lim::CH_RX_LIM_SPEC>;
#[doc = "Channel %s Rx event configuration register"]
pub mod ch_rx_lim;
#[doc = "SYS_CONF (rw) register accessor: an alias for `Reg<SYS_CONF_SPEC>`"]
pub type SYS_CONF = crate::Reg<sys_conf::SYS_CONF_SPEC>;
#[doc = "RMT apb configuration register"]
pub mod sys_conf;
#[doc = "TX_SIM (rw) register accessor: an alias for `Reg<TX_SIM_SPEC>`"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT TX synchronous register"]
pub mod tx_sim;
#[doc = "REF_CNT_RST (w) register accessor: an alias for `Reg<REF_CNT_RST_SPEC>`"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT clock divider reset register"]
pub mod ref_cnt_rst;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT version register"]
pub mod date;
