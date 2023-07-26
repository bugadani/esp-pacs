#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_TX_END[0-3]` reader - The interrupt enable bit for CH%s_TX_END_INT."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END[0-3]` writer - The interrupt enable bit for CH%s_TX_END_INT."]
pub type CH_TX_END_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_TX_ERR[0-3]` reader - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR[0-3]` writer - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_TX_THR_EVENT[0-3]` reader - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT[0-3]` writer - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_TX_LOOP[0-3]` reader - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP[0-3]` writer - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_RX_END[4-7]` reader - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END[4-7]` writer - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH_RX_END_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_RX_ERR[4-7]` reader - The interrupt enable bit for CH4_ERR_INT."]
pub type CH_RX_ERR_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR[4-7]` writer - The interrupt enable bit for CH4_ERR_INT."]
pub type CH_RX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_RX_THR_EVENT[4-7]` reader - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_RX_THR_EVENT[4-7]` writer - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH_RX_THR_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "The interrupt enable bit for CH[0-3]_TX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_err(&self, n: u8) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_LOOP_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> (n - 4 + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_err(&self, n: u8) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> (n - 4 + 20)) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch5_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch6_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch7_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_thr_event(&self, n: u8) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> (n - 4 + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail(&self) -> TX_CH3_DMA_ACCESS_FAIL_R {
        TX_CH3_DMA_ACCESS_FAIL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail(&self) -> RX_CH7_DMA_ACCESS_FAIL_R {
        RX_CH7_DMA_ACCESS_FAIL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("ch0_tx_end", &format_args!("{}", self.ch0_tx_end().bit()))
            .field("ch1_tx_end", &format_args!("{}", self.ch1_tx_end().bit()))
            .field("ch2_tx_end", &format_args!("{}", self.ch2_tx_end().bit()))
            .field("ch3_tx_end", &format_args!("{}", self.ch3_tx_end().bit()))
            .field("ch0_tx_err", &format_args!("{}", self.ch0_tx_err().bit()))
            .field("ch1_tx_err", &format_args!("{}", self.ch1_tx_err().bit()))
            .field("ch2_tx_err", &format_args!("{}", self.ch2_tx_err().bit()))
            .field("ch3_tx_err", &format_args!("{}", self.ch3_tx_err().bit()))
            .field(
                "ch0_tx_thr_event",
                &format_args!("{}", self.ch0_tx_thr_event().bit()),
            )
            .field(
                "ch1_tx_thr_event",
                &format_args!("{}", self.ch1_tx_thr_event().bit()),
            )
            .field(
                "ch2_tx_thr_event",
                &format_args!("{}", self.ch2_tx_thr_event().bit()),
            )
            .field(
                "ch3_tx_thr_event",
                &format_args!("{}", self.ch3_tx_thr_event().bit()),
            )
            .field("ch0_tx_loop", &format_args!("{}", self.ch0_tx_loop().bit()))
            .field("ch1_tx_loop", &format_args!("{}", self.ch1_tx_loop().bit()))
            .field("ch2_tx_loop", &format_args!("{}", self.ch2_tx_loop().bit()))
            .field("ch3_tx_loop", &format_args!("{}", self.ch3_tx_loop().bit()))
            .field("ch4_rx_end", &format_args!("{}", self.ch4_rx_end().bit()))
            .field("ch5_rx_end", &format_args!("{}", self.ch5_rx_end().bit()))
            .field("ch6_rx_end", &format_args!("{}", self.ch6_rx_end().bit()))
            .field("ch7_rx_end", &format_args!("{}", self.ch7_rx_end().bit()))
            .field("ch4_rx_err", &format_args!("{}", self.ch4_rx_err().bit()))
            .field("ch5_rx_err", &format_args!("{}", self.ch5_rx_err().bit()))
            .field("ch6_rx_err", &format_args!("{}", self.ch6_rx_err().bit()))
            .field("ch7_rx_err", &format_args!("{}", self.ch7_rx_err().bit()))
            .field(
                "ch4_rx_thr_event",
                &format_args!("{}", self.ch4_rx_thr_event().bit()),
            )
            .field(
                "ch5_rx_thr_event",
                &format_args!("{}", self.ch5_rx_thr_event().bit()),
            )
            .field(
                "ch6_rx_thr_event",
                &format_args!("{}", self.ch6_rx_thr_event().bit()),
            )
            .field(
                "ch7_rx_thr_event",
                &format_args!("{}", self.ch7_rx_thr_event().bit()),
            )
            .field(
                "tx_ch3_dma_access_fail",
                &format_args!("{}", self.tx_ch3_dma_access_fail().bit()),
            )
            .field(
                "rx_ch7_dma_access_fail",
                &format_args!("{}", self.rx_ch7_dma_access_fail().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "The interrupt enable bit for CH[0-3]_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end<const O: u8>(&mut self) -> CH_TX_END_W<O> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<0> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<1> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<2> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<3> {
        CH_TX_END_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err<const O: u8>(&mut self) -> CH_TX_ERR_W<O> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<4> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<5> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err(&mut self) -> CH_TX_ERR_W<6> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err(&mut self) -> CH_TX_ERR_W<7> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event<const O: u8>(&mut self) -> CH_TX_THR_EVENT_W<O> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<8> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<9> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<10> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<11> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop<const O: u8>(&mut self) -> CH_TX_LOOP_W<O> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<12> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<13> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop(&mut self) -> CH_TX_LOOP_W<14> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop(&mut self) -> CH_TX_LOOP_W<15> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end<const O: u8>(&mut self) -> CH_RX_END_W<O> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH_RX_END_W<16> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH_RX_END_W<17> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH_RX_END_W<18> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH_RX_END_W<19> {
        CH_RX_END_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err<const O: u8>(&mut self) -> CH_RX_ERR_W<O> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_err(&mut self) -> CH_RX_ERR_W<20> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_err(&mut self) -> CH_RX_ERR_W<21> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_err(&mut self) -> CH_RX_ERR_W<22> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_err(&mut self) -> CH_RX_ERR_W<23> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_thr_event<const O: u8>(&mut self) -> CH_RX_THR_EVENT_W<O> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<24> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<25> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<26> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<27> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<28> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<29> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
