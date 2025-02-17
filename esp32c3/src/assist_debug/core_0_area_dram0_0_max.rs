#[doc = "Register `CORE_0_AREA_DRAM0_0_MAX` reader"]
pub type R = crate::R<CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "Register `CORE_0_AREA_DRAM0_0_MAX` writer"]
pub type W = crate::W<CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_MAX` reader - reg_core_0_area_dram0_0_max"]
pub type CORE_0_AREA_DRAM0_0_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_MAX` writer - reg_core_0_area_dram0_0_max"]
pub type CORE_0_AREA_DRAM0_0_MAX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_area_dram0_0_max"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_max(&self) -> CORE_0_AREA_DRAM0_0_MAX_R {
        CORE_0_AREA_DRAM0_0_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_AREA_DRAM0_0_MAX")
            .field(
                "core_0_area_dram0_0_max",
                &format_args!("{}", self.core_0_area_dram0_0_max().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_AREA_DRAM0_0_MAX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_core_0_area_dram0_0_max"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_max(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_MAX_W<CORE_0_AREA_DRAM0_0_MAX_SPEC, 0> {
        CORE_0_AREA_DRAM0_0_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_0_max::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_0_max::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_AREA_DRAM0_0_MAX_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_DRAM0_0_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_area_dram0_0_max::R`](R) reader structure"]
impl crate::Readable for CORE_0_AREA_DRAM0_0_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_area_dram0_0_max::W`](W) writer structure"]
impl crate::Writable for CORE_0_AREA_DRAM0_0_MAX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_AREA_DRAM0_0_MAX to value 0"]
impl crate::Resettable for CORE_0_AREA_DRAM0_0_MAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
