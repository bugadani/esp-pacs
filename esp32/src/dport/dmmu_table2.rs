#[doc = "Register `DMMU_TABLE2` reader"]
pub type R = crate::R<DMMU_TABLE2_SPEC>;
#[doc = "Register `DMMU_TABLE2` writer"]
pub type W = crate::W<DMMU_TABLE2_SPEC>;
#[doc = "Field `DMMU_TABLE2` reader - "]
pub type DMMU_TABLE2_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE2` writer - "]
pub type DMMU_TABLE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table2(&self) -> DMMU_TABLE2_R {
        DMMU_TABLE2_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE2")
            .field(
                "dmmu_table2",
                &format_args!("{}", self.dmmu_table2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_TABLE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table2(&mut self) -> DMMU_TABLE2_W<DMMU_TABLE2_SPEC, 0> {
        DMMU_TABLE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE2_SPEC;
impl crate::RegisterSpec for DMMU_TABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table2::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table2::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE2 to value 0x02"]
impl crate::Resettable for DMMU_TABLE2_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
