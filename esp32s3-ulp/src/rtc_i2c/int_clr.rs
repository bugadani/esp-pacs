#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLAVE_TRAN_COMP_INT_CLR` writer - clear slave transit complete interrupt"]
pub type SLAVE_TRAN_COMP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - clear arbitration lost interrupt"]
pub type ARBITRATION_LOST_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASTER_TRAN_COMP_INT_CLR` writer - clear master transit complete interrupt"]
pub type MASTER_TRAN_COMP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - clear transit complete interrupt"]
pub type TRANS_COMPLETE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIME_OUT_INT_CLR` writer - clear time out interrupt"]
pub type TIME_OUT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACK_ERR_INT_CLR` writer - clear ack error interrupt"]
pub type ACK_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_DATA_INT_CLR` writer - clear receive data interrupt"]
pub type RX_DATA_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_DATA_INT_CLR` writer - clear transit load data complete interrupt"]
pub type TX_DATA_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DETECT_START_INT_CLR` writer - clear detect start interrupt"]
pub type DETECT_START_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - clear slave transit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slave_tran_comp_int_clr(&mut self) -> SLAVE_TRAN_COMP_INT_CLR_W<INT_CLR_SPEC, 0> {
        SLAVE_TRAN_COMP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - clear arbitration lost interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W<INT_CLR_SPEC, 1> {
        ARBITRATION_LOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - clear master transit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn master_tran_comp_int_clr(&mut self) -> MASTER_TRAN_COMP_INT_CLR_W<INT_CLR_SPEC, 2> {
        MASTER_TRAN_COMP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - clear transit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W<INT_CLR_SPEC, 3> {
        TRANS_COMPLETE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - clear time out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W<INT_CLR_SPEC, 4> {
        TIME_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - clear ack error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack_err_int_clr(&mut self) -> ACK_ERR_INT_CLR_W<INT_CLR_SPEC, 5> {
        ACK_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - clear receive data interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_int_clr(&mut self) -> RX_DATA_INT_CLR_W<INT_CLR_SPEC, 6> {
        RX_DATA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - clear transit load data complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_int_clr(&mut self) -> TX_DATA_INT_CLR_W<INT_CLR_SPEC, 7> {
        TX_DATA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - clear detect start interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn detect_start_int_clr(&mut self) -> DETECT_START_INT_CLR_W<INT_CLR_SPEC, 8> {
        DETECT_START_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
