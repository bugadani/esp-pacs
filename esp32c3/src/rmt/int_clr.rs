#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `CH_TX_END[0-1]` writer - reg_ch%s_tx_end_int_clr."]
pub type CH_TX_END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_END[2-3]` writer - reg_ch2_rx_end_int_clr."]
pub type CH_RX_END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_ERR[0-1]` writer - reg_ch%s_err_int_clr."]
pub type CH_TX_ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_ERR[2-3]` writer - reg_ch2_err_int_clr."]
pub type CH_RX_ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_THR_EVENT[0-1]` writer - reg_ch%s_tx_thr_event_int_clr."]
pub type CH_TX_THR_EVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_THR_EVENT[2-3]` writer - reg_ch2_rx_thr_event_int_clr."]
pub type CH_RX_THR_EVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_LOOP[0-1]` writer - reg_ch%s_tx_loop_int_clr."]
pub type CH_TX_LOOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "reg_ch[0-1]_tx_end_int_clr."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end<const O: u8>(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, O> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, 0> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, 1> {
        CH_TX_END_W::new(self)
    }
    #[doc = "reg_ch2_rx_end_int_clr."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end<const O: u8>(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, O> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, 2> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, 3> {
        CH_RX_END_W::new(self)
    }
    #[doc = "reg_ch[0-1]_err_int_clr."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err<const O: u8>(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, O> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 4 - reg_ch0_err_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, 4> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, 5> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "reg_ch2_err_int_clr."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err<const O: u8>(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, O> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 6 - reg_ch2_err_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, 6> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, 7> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "reg_ch[0-1]_tx_thr_event_int_clr."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event<const O: u8>(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, O> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, 8> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, 9> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "reg_ch2_rx_thr_event_int_clr."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_thr_event<const O: u8>(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, O> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, 10> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 11 - reg_ch2_rx_thr_event_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, 11> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "reg_ch[0-1]_tx_loop_int_clr."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop<const O: u8>(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, O> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, 12> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, 13> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RMT_INT_CLR_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
