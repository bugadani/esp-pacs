#[doc = "Register `DMMU_TABLE13` reader"]
pub type R = crate::R<DMMU_TABLE13_SPEC>;
#[doc = "Register `DMMU_TABLE13` writer"]
pub type W = crate::W<DMMU_TABLE13_SPEC>;
#[doc = "Field `DMMU_TABLE13` reader - "]
pub type DMMU_TABLE13_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE13` writer - "]
pub type DMMU_TABLE13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table13(&self) -> DMMU_TABLE13_R {
        DMMU_TABLE13_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE13")
            .field(
                "dmmu_table13",
                &format_args!("{}", self.dmmu_table13().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_TABLE13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table13(&mut self) -> DMMU_TABLE13_W<DMMU_TABLE13_SPEC, 0> {
        DMMU_TABLE13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE13_SPEC;
impl crate::RegisterSpec for DMMU_TABLE13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table13::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table13::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE13 to value 0x0d"]
impl crate::Resettable for DMMU_TABLE13_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
