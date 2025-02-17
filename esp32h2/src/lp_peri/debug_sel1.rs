#[doc = "Register `DEBUG_SEL1` reader"]
pub type R = crate::R<DEBUG_SEL1_SPEC>;
#[doc = "Register `DEBUG_SEL1` writer"]
pub type W = crate::W<DEBUG_SEL1_SPEC>;
#[doc = "Field `DEBUG_SEL4` reader - need des"]
pub type DEBUG_SEL4_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL4` writer - need des"]
pub type DEBUG_SEL4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    pub fn debug_sel4(&self) -> DEBUG_SEL4_R {
        DEBUG_SEL4_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL1")
            .field("debug_sel4", &format_args!("{}", self.debug_sel4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEBUG_SEL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel4(&mut self) -> DEBUG_SEL4_W<DEBUG_SEL1_SPEC, 0> {
        DEBUG_SEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_sel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SEL1_SPEC;
impl crate::RegisterSpec for DEBUG_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_sel1::R`](R) reader structure"]
impl crate::Readable for DEBUG_SEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_sel1::W`](W) writer structure"]
impl crate::Writable for DEBUG_SEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL1 to value 0"]
impl crate::Resettable for DEBUG_SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
