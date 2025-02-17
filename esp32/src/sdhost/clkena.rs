#[doc = "Register `CLKENA` reader"]
pub type R = crate::R<CLKENA_SPEC>;
#[doc = "Register `CLKENA` writer"]
pub type W = crate::W<CLKENA_SPEC>;
#[doc = "Field `CCLK_ENABLE` reader - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
pub type CCLK_ENABLE_R = crate::FieldReader;
#[doc = "Field `CCLK_ENABLE` writer - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
pub type CCLK_ENABLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LP_ENABLE` reader - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
pub type LP_ENABLE_R = crate::FieldReader;
#[doc = "Field `LP_ENABLE` writer - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
pub type LP_ENABLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
    #[inline(always)]
    pub fn cclk_enable(&self) -> CCLK_ENABLE_R {
        CCLK_ENABLE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
    #[inline(always)]
    pub fn lp_enable(&self) -> LP_ENABLE_R {
        LP_ENABLE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKENA")
            .field(
                "cclk_enable",
                &format_args!("{}", self.cclk_enable().bits()),
            )
            .field("lp_enable", &format_args!("{}", self.lp_enable().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_enable(&mut self) -> CCLK_ENABLE_W<CLKENA_SPEC, 0> {
        CCLK_ENABLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lp_enable(&mut self) -> LP_ENABLE_W<CLKENA_SPEC, 16> {
        LP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKENA_SPEC;
impl crate::RegisterSpec for CLKENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkena::R`](R) reader structure"]
impl crate::Readable for CLKENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkena::W`](W) writer structure"]
impl crate::Writable for CLKENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKENA to value 0"]
impl crate::Resettable for CLKENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
