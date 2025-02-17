#[doc = "Register `REDUNDANCY_SIG3` reader"]
pub type R = crate::R<REDUNDANCY_SIG3_SPEC>;
#[doc = "Register `REDUNDANCY_SIG3` writer"]
pub type W = crate::W<REDUNDANCY_SIG3_SPEC>;
#[doc = "Field `CACHE_REDCY_SIG3` reader - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG3_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_REDCY_SIG3` writer - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn cache_redcy_sig3(&self) -> CACHE_REDCY_SIG3_R {
        CACHE_REDCY_SIG3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDUNDANCY_SIG3")
            .field(
                "cache_redcy_sig3",
                &format_args!("{}", self.cache_redcy_sig3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDUNDANCY_SIG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn cache_redcy_sig3(&mut self) -> CACHE_REDCY_SIG3_W<REDUNDANCY_SIG3_SPEC, 0> {
        CACHE_REDCY_SIG3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache redundancy signal 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REDUNDANCY_SIG3_SPEC;
impl crate::RegisterSpec for REDUNDANCY_SIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redundancy_sig3::R`](R) reader structure"]
impl crate::Readable for REDUNDANCY_SIG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`redundancy_sig3::W`](W) writer structure"]
impl crate::Writable for REDUNDANCY_SIG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REDUNDANCY_SIG3 to value 0"]
impl crate::Resettable for REDUNDANCY_SIG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
