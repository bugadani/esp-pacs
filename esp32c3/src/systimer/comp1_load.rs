#[doc = "Register `COMP1_LOAD` writer"]
pub type W = crate::W<COMP1_LOAD_SPEC>;
#[doc = "Field `TIMER_COMP1_LOAD` writer - timer comp1 load value"]
pub type TIMER_COMP1_LOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMP1_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - timer comp1 load value"]
    #[inline(always)]
    #[must_use]
    pub fn timer_comp1_load(&mut self) -> TIMER_COMP1_LOAD_W<COMP1_LOAD_SPEC, 0> {
        TIMER_COMP1_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSTIMER_COMP1_LOAD.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP1_LOAD_SPEC;
impl crate::RegisterSpec for COMP1_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`comp1_load::W`](W) writer structure"]
impl crate::Writable for COMP1_LOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP1_LOAD to value 0"]
impl crate::Resettable for COMP1_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
