#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `CH_TX_END[0-1]` reader - reg_ch%s_tx_end_int_raw."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END[0-1]` writer - reg_ch%s_tx_end_int_raw."]
pub type CH_TX_END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_END[2-3]` reader - reg_ch2_rx_end_int_raw."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END[2-3]` writer - reg_ch2_rx_end_int_raw."]
pub type CH_RX_END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_ERR[0-1]` reader - reg_ch%s_err_int_raw."]
pub type CH_TX_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR[0-1]` writer - reg_ch%s_err_int_raw."]
pub type CH_TX_ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_ERR[2-3]` reader - reg_ch2_err_int_raw."]
pub type CH_RX_ERR_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR[2-3]` writer - reg_ch2_err_int_raw."]
pub type CH_RX_ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_THR_EVENT[0-1]` reader - reg_ch%s_tx_thr_event_int_raw."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT[0-1]` writer - reg_ch%s_tx_thr_event_int_raw."]
pub type CH_TX_THR_EVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_RX_THR_EVENT[2-3]` reader - reg_ch2_rx_thr_event_int_raw."]
pub type CH_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_RX_THR_EVENT[2-3]` writer - reg_ch2_rx_thr_event_int_raw."]
pub type CH_RX_THR_EVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_TX_LOOP[0-1]` reader - reg_ch%s_tx_loop_int_raw."]
pub type CH_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP[0-1]` writer - reg_ch%s_tx_loop_int_raw."]
pub type CH_TX_LOOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "reg_ch[0-1]_tx_end_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> (n - 2 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_err_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_err(&self, n: u8) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_ch0_err_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "reg_ch2_err_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_rx_err(&self, n: u8) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> (n - 2 + 6)) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_ch2_err_int_raw."]
    #[inline(always)]
    pub fn ch2_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_raw."]
    #[inline(always)]
    pub fn ch3_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_tx_thr_event_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "reg_ch2_rx_thr_event_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_rx_thr_event(&self, n: u8) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> (n - 2 + 10)) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch2_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ch2_rx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch3_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_tx_loop_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("ch0_tx_end", &format_args!("{}", self.ch0_tx_end().bit()))
            .field("ch1_tx_end", &format_args!("{}", self.ch1_tx_end().bit()))
            .field("ch2_rx_end", &format_args!("{}", self.ch2_rx_end().bit()))
            .field("ch3_rx_end", &format_args!("{}", self.ch3_rx_end().bit()))
            .field("ch0_tx_err", &format_args!("{}", self.ch0_tx_err().bit()))
            .field("ch1_tx_err", &format_args!("{}", self.ch1_tx_err().bit()))
            .field("ch2_rx_err", &format_args!("{}", self.ch2_rx_err().bit()))
            .field("ch3_rx_err", &format_args!("{}", self.ch3_rx_err().bit()))
            .field(
                "ch0_tx_thr_event",
                &format_args!("{}", self.ch0_tx_thr_event().bit()),
            )
            .field(
                "ch1_tx_thr_event",
                &format_args!("{}", self.ch1_tx_thr_event().bit()),
            )
            .field(
                "ch2_rx_thr_event",
                &format_args!("{}", self.ch2_rx_thr_event().bit()),
            )
            .field(
                "ch3_rx_thr_event",
                &format_args!("{}", self.ch3_rx_thr_event().bit()),
            )
            .field("ch0_tx_loop", &format_args!("{}", self.ch0_tx_loop().bit()))
            .field("ch1_tx_loop", &format_args!("{}", self.ch1_tx_loop().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "reg_ch[0-1]_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end<const O: u8>(&mut self) -> CH_TX_END_W<INT_RAW_SPEC, O> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC, 0> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC, 1> {
        CH_TX_END_W::new(self)
    }
    #[doc = "reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end<const O: u8>(&mut self) -> CH_RX_END_W<INT_RAW_SPEC, O> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC, 2> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC, 3> {
        CH_RX_END_W::new(self)
    }
    #[doc = "reg_ch[0-1]_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err<const O: u8>(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC, O> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 4 - reg_ch0_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC, 4> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC, 5> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "reg_ch2_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err<const O: u8>(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC, O> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 6 - reg_ch2_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC, 6> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC, 7> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "reg_ch[0-1]_tx_thr_event_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event<const O: u8>(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC, O> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC, 8> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC, 9> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "reg_ch2_rx_thr_event_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_thr_event<const O: u8>(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC, O> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC, 10> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 11 - reg_ch2_rx_thr_event_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC, 11> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "reg_ch[0-1]_tx_loop_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop<const O: u8>(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC, O> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC, 12> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC, 13> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RMT_INT_RAW_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
