#[doc = "Register `TIMERS_DATE` reader"]
pub type R = crate::R<TIMERS_DATE_SPEC>;
#[doc = "Register `TIMERS_DATE` writer"]
pub type W = crate::W<TIMERS_DATE_SPEC>;
#[doc = "Field `TIMERS_DATE` reader - Version control register."]
pub type TIMERS_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `TIMERS_DATE` writer - Version control register."]
pub type TIMERS_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn timers_date(&self) -> TIMERS_DATE_R {
        TIMERS_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERS_DATE")
            .field(
                "timers_date",
                &format_args!("{}", self.timers_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMERS_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    #[must_use]
    pub fn timers_date(&mut self) -> TIMERS_DATE_W<TIMERS_DATE_SPEC, 0> {
        TIMERS_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timers_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timers_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERS_DATE_SPEC;
impl crate::RegisterSpec for TIMERS_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timers_date::R`](R) reader structure"]
impl crate::Readable for TIMERS_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timers_date::W`](W) writer structure"]
impl crate::Writable for TIMERS_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMERS_DATE to value 0x0190_7261"]
impl crate::Resettable for TIMERS_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0190_7261;
}
