#[doc = "Register `TOUCH_APPROACH` reader"]
pub type R = crate::R<TOUCH_APPROACH_SPEC>;
#[doc = "Register `TOUCH_APPROACH` writer"]
pub type W = crate::W<TOUCH_APPROACH_SPEC>;
#[doc = "Field `TOUCH_SLP_CHANNEL_CLR` writer - Clear touch sleep channel."]
pub type TOUCH_SLP_CHANNEL_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEAS_TIME` reader - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
pub type MEAS_TIME_R = crate::FieldReader;
#[doc = "Field `MEAS_TIME` writer - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
pub type MEAS_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 24:31 - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
    #[inline(always)]
    pub fn meas_time(&self) -> MEAS_TIME_R {
        MEAS_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_APPROACH")
            .field("meas_time", &format_args!("{}", self.meas_time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_APPROACH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 23 - Clear touch sleep channel."]
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_channel_clr(&mut self) -> TOUCH_SLP_CHANNEL_CLR_W<TOUCH_APPROACH_SPEC, 23> {
        TOUCH_SLP_CHANNEL_CLR_W::new(self)
    }
    #[doc = "Bits 24:31 - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
    #[inline(always)]
    #[must_use]
    pub fn meas_time(&mut self) -> MEAS_TIME_W<TOUCH_APPROACH_SPEC, 24> {
        MEAS_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure touch approach settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_approach::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_APPROACH_SPEC;
impl crate::RegisterSpec for TOUCH_APPROACH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_approach::R`](R) reader structure"]
impl crate::Readable for TOUCH_APPROACH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_approach::W`](W) writer structure"]
impl crate::Writable for TOUCH_APPROACH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_APPROACH to value 0x5000_0000"]
impl crate::Resettable for TOUCH_APPROACH_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000_0000;
}
