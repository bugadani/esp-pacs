#[doc = "Register `REGION_PMS_CONSTRAIN_3` reader"]
pub type R = crate::R<REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Register `REGION_PMS_CONSTRAIN_3` writer"]
pub type W = crate::W<REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Field `REGION_PMS_CONSTRAIN_ADDR_0` reader - region_pms_constrain_addr_0"]
pub type REGION_PMS_CONSTRAIN_ADDR_0_R = crate::FieldReader<u32>;
#[doc = "Field `REGION_PMS_CONSTRAIN_ADDR_0` writer - region_pms_constrain_addr_0"]
pub type REGION_PMS_CONSTRAIN_ADDR_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29 - region_pms_constrain_addr_0"]
    #[inline(always)]
    pub fn region_pms_constrain_addr_0(&self) -> REGION_PMS_CONSTRAIN_ADDR_0_R {
        REGION_PMS_CONSTRAIN_ADDR_0_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_PMS_CONSTRAIN_3")
            .field(
                "region_pms_constrain_addr_0",
                &format_args!("{}", self.region_pms_constrain_addr_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION_PMS_CONSTRAIN_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:29 - region_pms_constrain_addr_0"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_addr_0(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_ADDR_0_W<REGION_PMS_CONSTRAIN_3_SPEC, 0> {
        REGION_PMS_CONSTRAIN_ADDR_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region_pms_constrain_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_pms_constrain_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for REGION_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_pms_constrain_3::R`](R) reader structure"]
impl crate::Readable for REGION_PMS_CONSTRAIN_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region_pms_constrain_3::W`](W) writer structure"]
impl crate::Writable for REGION_PMS_CONSTRAIN_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION_PMS_CONSTRAIN_3 to value 0"]
impl crate::Resettable for REGION_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
