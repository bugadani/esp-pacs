#[doc = "Register `X_MEM[%s]` writer"]
pub type W = crate::W<X_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<X_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Represents X\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct X_MEM_SPEC;
impl crate::RegisterSpec for X_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`x_mem::W`](W) writer structure"]
impl crate::Writable for X_MEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets X_MEM[%s] to value 0"]
impl crate::Resettable for X_MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
