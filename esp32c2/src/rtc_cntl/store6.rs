#[doc = "Register `STORE6` reader"]
pub type R = crate::R<STORE6_SPEC>;
#[doc = "Register `STORE6` writer"]
pub type W = crate::W<STORE6_SPEC>;
#[doc = "Field `SCRATCH6` reader - Need add desc"]
pub type SCRATCH6_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH6` writer - Need add desc"]
pub type SCRATCH6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn scratch6(&self) -> SCRATCH6_R {
        SCRATCH6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE6")
            .field("scratch6", &format_args!("{}", self.scratch6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn scratch6(&mut self) -> SCRATCH6_W<STORE6_SPEC, 0> {
        SCRATCH6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE6_SPEC;
impl crate::RegisterSpec for STORE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store6::R`](R) reader structure"]
impl crate::Readable for STORE6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store6::W`](W) writer structure"]
impl crate::Writable for STORE6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE6 to value 0"]
impl crate::Resettable for STORE6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
