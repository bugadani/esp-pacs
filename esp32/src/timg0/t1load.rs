#[doc = "Register `T1LOAD` writer"]
pub type W = crate::W<T1LOAD_SPEC>;
#[doc = "Field `LOAD` writer - Write any value will trigger timer 1 time-base counter reload"]
pub type LOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value will trigger timer 1 time-base counter reload"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<T1LOAD_SPEC, 0> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1LOAD_SPEC;
impl crate::RegisterSpec for T1LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`t1load::W`](W) writer structure"]
impl crate::Writable for T1LOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T1LOAD to value 0"]
impl crate::Resettable for T1LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
