#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `CH_TX_END[0-3]` writer - Set this bit to clear theCH%s_TX_END_INT interrupt."]
pub type CH_TX_END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_ERR[0-3]` writer - Set this bit to clear theCH%s_ERR_INT interrupt."]
pub type CH_TX_ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_THR_EVENT[0-3]` writer - Set this bit to clear theCH%s_TX_THR_EVENT_INT interrupt."]
pub type CH_TX_THR_EVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_LOOP[0-3]` writer - Set this bit to clear theCH%s_TX_LOOP_INT interrupt."]
pub type CH_TX_LOOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_END[4-7]` writer - Set this bit to clear theCH4_RX_END_INT interrupt."]
pub type CH_RX_END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_ERR[4-7]` writer - Set this bit to clear theCH4_ERR_INT interrupt."]
pub type CH_RX_ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_THR_EVENT[4-7]` writer - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
pub type CH_RX_THR_EVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` writer - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt."]
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` writer - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt."]
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set this bit to clear theCH[0-3]_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end<const O: u8>(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, O> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 0 - Set this bit to clear theCH0_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, 0> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear theCH1_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, 1> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear theCH2_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, 2> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear theCH3_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC, 3> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-3]_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err<const O: u8>(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, O> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear theCH0_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, 4> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear theCH1_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, 5> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear theCH2_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, 6> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear theCH3_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC, 7> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-3]_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event<const O: u8>(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, O> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, 8> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, 9> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, 10> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC, 11> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-3]_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop<const O: u8>(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, O> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear theCH0_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, 12> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear theCH1_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, 13> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear theCH2_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, 14> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear theCH3_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC, 15> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end<const O: u8>(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, O> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, 16> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, 17> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, 18> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC, 19> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err<const O: u8>(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, O> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, 20> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, 21> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, 22> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC, 23> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_thr_event<const O: u8>(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, O> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, 24> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 25 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, 25> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, 26> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC, 27> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<INT_CLR_SPEC, 28> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<INT_CLR_SPEC, 29> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
