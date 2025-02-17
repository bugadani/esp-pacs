#[doc = "Register `CH%s_RX_LIM` reader"]
pub type R = crate::R<CH_RX_LIM_SPEC>;
#[doc = "Register `CH%s_RX_LIM` writer"]
pub type W = crate::W<CH_RX_LIM_SPEC>;
#[doc = "Field `RMT_RX_LIM` reader - This register is used to configure the maximum entries that CHANNEL%s can receive."]
pub type RMT_RX_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `RMT_RX_LIM` writer - This register is used to configure the maximum entries that CHANNEL%s can receive."]
pub type RMT_RX_LIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can receive."]
    #[inline(always)]
    pub fn rmt_rx_lim(&self) -> RMT_RX_LIM_R {
        RMT_RX_LIM_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_LIM")
            .field("rmt_rx_lim", &format_args!("{}", self.rmt_rx_lim().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_RX_LIM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can receive."]
    #[inline(always)]
    #[must_use]
    pub fn rmt_rx_lim(&mut self) -> RMT_RX_LIM_W<CH_RX_LIM_SPEC, 0> {
        RMT_RX_LIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel %s Rx event configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_lim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_lim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_LIM_SPEC;
impl crate::RegisterSpec for CH_RX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_lim::R`](R) reader structure"]
impl crate::Readable for CH_RX_LIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_lim::W`](W) writer structure"]
impl crate::Writable for CH_RX_LIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_RX_LIM to value 0x80"]
impl crate::Resettable for CH_RX_LIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
