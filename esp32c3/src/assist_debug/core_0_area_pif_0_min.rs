#[doc = "Register `CORE_0_AREA_PIF_0_MIN` reader"]
pub type R = crate::R<CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "Register `CORE_0_AREA_PIF_0_MIN` writer"]
pub type W = crate::W<CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "Field `CORE_0_AREA_PIF_0_MIN` reader - reg_core_0_area_pif_0_min"]
pub type CORE_0_AREA_PIF_0_MIN_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_PIF_0_MIN` writer - reg_core_0_area_pif_0_min"]
pub type CORE_0_AREA_PIF_0_MIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_area_pif_0_min"]
    #[inline(always)]
    pub fn core_0_area_pif_0_min(&self) -> CORE_0_AREA_PIF_0_MIN_R {
        CORE_0_AREA_PIF_0_MIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_AREA_PIF_0_MIN")
            .field(
                "core_0_area_pif_0_min",
                &format_args!("{}", self.core_0_area_pif_0_min().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_AREA_PIF_0_MIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_core_0_area_pif_0_min"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_0_min(
        &mut self,
    ) -> CORE_0_AREA_PIF_0_MIN_W<CORE_0_AREA_PIF_0_MIN_SPEC, 0> {
        CORE_0_AREA_PIF_0_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_0_min::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_0_min::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_AREA_PIF_0_MIN_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_PIF_0_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_area_pif_0_min::R`](R) reader structure"]
impl crate::Readable for CORE_0_AREA_PIF_0_MIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_area_pif_0_min::W`](W) writer structure"]
impl crate::Writable for CORE_0_AREA_PIF_0_MIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_AREA_PIF_0_MIN to value 0xffff_ffff"]
impl crate::Resettable for CORE_0_AREA_PIF_0_MIN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
