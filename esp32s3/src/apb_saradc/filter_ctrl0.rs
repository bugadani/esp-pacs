#[doc = "Register `FILTER_CTRL0` reader"]
pub type R = crate::R<FILTER_CTRL0_SPEC>;
#[doc = "Register `FILTER_CTRL0` writer"]
pub type W = crate::W<FILTER_CTRL0_SPEC>;
#[doc = "Field `FILTER_CHANNEL1` reader - configure the filter1 channel"]
pub type FILTER_CHANNEL1_R = crate::FieldReader;
#[doc = "Field `FILTER_CHANNEL1` writer - configure the filter1 channel"]
pub type FILTER_CHANNEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `FILTER_CHANNEL0` reader - configure the filter0 channel"]
pub type FILTER_CHANNEL0_R = crate::FieldReader;
#[doc = "Field `FILTER_CHANNEL0` writer - configure the filter0 channel"]
pub type FILTER_CHANNEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `FILTER_RESET` reader - enable apb_adc1_filter"]
pub type FILTER_RESET_R = crate::BitReader;
#[doc = "Field `FILTER_RESET` writer - enable apb_adc1_filter"]
pub type FILTER_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 14:18 - configure the filter1 channel"]
    #[inline(always)]
    pub fn filter_channel1(&self) -> FILTER_CHANNEL1_R {
        FILTER_CHANNEL1_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - configure the filter0 channel"]
    #[inline(always)]
    pub fn filter_channel0(&self) -> FILTER_CHANNEL0_R {
        FILTER_CHANNEL0_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    pub fn filter_reset(&self) -> FILTER_RESET_R {
        FILTER_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CTRL0")
            .field(
                "filter_channel1",
                &format_args!("{}", self.filter_channel1().bits()),
            )
            .field(
                "filter_channel0",
                &format_args!("{}", self.filter_channel0().bits()),
            )
            .field(
                "filter_reset",
                &format_args!("{}", self.filter_reset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 14:18 - configure the filter1 channel"]
    #[inline(always)]
    #[must_use]
    pub fn filter_channel1(&mut self) -> FILTER_CHANNEL1_W<FILTER_CTRL0_SPEC, 14> {
        FILTER_CHANNEL1_W::new(self)
    }
    #[doc = "Bits 19:23 - configure the filter0 channel"]
    #[inline(always)]
    #[must_use]
    pub fn filter_channel0(&mut self) -> FILTER_CHANNEL0_W<FILTER_CTRL0_SPEC, 19> {
        FILTER_CHANNEL0_W::new(self)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter_reset(&mut self) -> FILTER_RESET_W<FILTER_CTRL0_SPEC, 31> {
        FILTER_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure apb saradc arbit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_CTRL0_SPEC;
impl crate::RegisterSpec for FILTER_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_ctrl0::R`](R) reader structure"]
impl crate::Readable for FILTER_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_ctrl0::W`](W) writer structure"]
impl crate::Writable for FILTER_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER_CTRL0 to value 0x006b_4000"]
impl crate::Resettable for FILTER_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x006b_4000;
}
