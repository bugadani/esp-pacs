#[doc = "Register `DOEPDMA0` reader"]
pub type R = crate::R<DOEPDMA0_SPEC>;
#[doc = "Register `DOEPDMA0` writer"]
pub type W = crate::W<DOEPDMA0_SPEC>;
#[doc = "Field `DMAADDR0` reader - "]
pub type DMAADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR0` writer - "]
pub type DMAADDR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr0(&self) -> DMAADDR0_R {
        DMAADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA0")
            .field("dmaaddr0", &format_args!("{}", self.dmaaddr0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr0(&mut self) -> DMAADDR0_W<DOEPDMA0_SPEC, 0> {
        DMAADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA0_SPEC;
impl crate::RegisterSpec for DOEPDMA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma0::R`](R) reader structure"]
impl crate::Readable for DOEPDMA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma0::W`](W) writer structure"]
impl crate::Writable for DOEPDMA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA0 to value 0"]
impl crate::Resettable for DOEPDMA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
