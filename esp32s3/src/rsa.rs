#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x200 - Memory M"]
    pub m_mem: [M_MEM; 512],
    #[doc = "0x200..0x400 - Memory Z"]
    pub z_mem: [Z_MEM; 512],
    #[doc = "0x400..0x600 - Memory Y"]
    pub y_mem: [Y_MEM; 512],
    #[doc = "0x600..0x800 - Memory X"]
    pub x_mem: [X_MEM; 512],
    #[doc = "0x800 - RSA M' register"]
    pub m_prime: M_PRIME,
    #[doc = "0x804 - RSA length mode register"]
    pub mode: MODE,
    #[doc = "0x808 - RSA clean register"]
    pub clean: CLEAN,
    #[doc = "0x80c - Modular exponentiation trigger register."]
    pub modexp_start: MODEXP_START,
    #[doc = "0x810 - Modular multiplication trigger register."]
    pub modmult_start: MODMULT_START,
    #[doc = "0x814 - Normal multiplication trigger register."]
    pub mult_start: MULT_START,
    #[doc = "0x818 - RSA idle register"]
    pub idle: IDLE,
    #[doc = "0x81c - RSA interrupt clear register"]
    pub clear_interrupt: CLEAR_INTERRUPT,
    #[doc = "0x820 - CONSTANT_TIME option control register"]
    pub constant_time: CONSTANT_TIME,
    #[doc = "0x824 - SEARCH option enable register"]
    pub search_enable: SEARCH_ENABLE,
    #[doc = "0x828 - RSA search position configure register"]
    pub search_pos: SEARCH_POS,
    #[doc = "0x82c - RSA interrupt enable register"]
    pub interrupt_ena: INTERRUPT_ENA,
    #[doc = "0x830 - RSA version control register"]
    pub date: DATE,
}
#[doc = "M_MEM (w) register accessor: Memory M\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_mem`] module"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Memory M"]
pub mod m_mem;
#[doc = "Z_MEM (rw) register accessor: Memory Z\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`z_mem`] module"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "Memory Z"]
pub mod z_mem;
#[doc = "Y_MEM (w) register accessor: Memory Y\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`y_mem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`y_mem`] module"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "Memory Y"]
pub mod y_mem;
#[doc = "X_MEM (w) register accessor: Memory X\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`x_mem`] module"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "Memory X"]
pub mod x_mem;
#[doc = "M_PRIME (rw) register accessor: RSA M' register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_prime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_prime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_prime`] module"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = "RSA M' register"]
pub mod m_prime;
#[doc = "MODE (rw) register accessor: RSA length mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "RSA length mode register"]
pub mod mode;
#[doc = "CLEAN (r) register accessor: RSA clean register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clean::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clean`] module"]
pub type CLEAN = crate::Reg<clean::CLEAN_SPEC>;
#[doc = "RSA clean register"]
pub mod clean;
#[doc = "MODEXP_START (w) register accessor: Modular exponentiation trigger register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modexp_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`modexp_start`] module"]
pub type MODEXP_START = crate::Reg<modexp_start::MODEXP_START_SPEC>;
#[doc = "Modular exponentiation trigger register."]
pub mod modexp_start;
#[doc = "MODMULT_START (w) register accessor: Modular multiplication trigger register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modmult_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`modmult_start`] module"]
pub type MODMULT_START = crate::Reg<modmult_start::MODMULT_START_SPEC>;
#[doc = "Modular multiplication trigger register."]
pub mod modmult_start;
#[doc = "MULT_START (w) register accessor: Normal multiplication trigger register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_start`] module"]
pub type MULT_START = crate::Reg<mult_start::MULT_START_SPEC>;
#[doc = "Normal multiplication trigger register."]
pub mod mult_start;
#[doc = "IDLE (r) register accessor: RSA idle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idle`] module"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = "RSA idle register"]
pub mod idle;
#[doc = "CLEAR_INTERRUPT (w) register accessor: RSA interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_interrupt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clear_interrupt`] module"]
pub type CLEAR_INTERRUPT = crate::Reg<clear_interrupt::CLEAR_INTERRUPT_SPEC>;
#[doc = "RSA interrupt clear register"]
pub mod clear_interrupt;
#[doc = "CONSTANT_TIME (rw) register accessor: CONSTANT_TIME option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`constant_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`constant_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`constant_time`] module"]
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
#[doc = "CONSTANT_TIME option control register"]
pub mod constant_time;
#[doc = "SEARCH_ENABLE (rw) register accessor: SEARCH option enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`search_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`search_enable`] module"]
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
#[doc = "SEARCH option enable register"]
pub mod search_enable;
#[doc = "SEARCH_POS (rw) register accessor: RSA search position configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`search_pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`search_pos`] module"]
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
#[doc = "RSA search position configure register"]
pub mod search_pos;
#[doc = "INTERRUPT_ENA (rw) register accessor: RSA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`interrupt_ena`] module"]
pub type INTERRUPT_ENA = crate::Reg<interrupt_ena::INTERRUPT_ENA_SPEC>;
#[doc = "RSA interrupt enable register"]
pub mod interrupt_ena;
#[doc = "DATE (rw) register accessor: RSA version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RSA version control register"]
pub mod date;
