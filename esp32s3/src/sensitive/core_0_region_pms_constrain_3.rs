#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_3` reader"]
pub type R = crate::R<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_3` writer"]
pub type W = crate::W<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_0` reader - Region 0 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_0` writer - Region 0 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29 - Region 0 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_0(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_REGION_PMS_CONSTRAIN_3")
            .field(
                "core_0_region_pms_constrain_addr_0",
                &format_args!("{}", self.core_0_region_pms_constrain_addr_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 0 start address for core0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_region_pms_constrain_addr_0(
        &mut self,
    ) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_W<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC, 0> {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core0 region permission register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_REGION_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_region_pms_constrain_3::R`](R) reader structure"]
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_region_pms_constrain_3::W`](W) writer structure"]
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_3 to value 0"]
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
