#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_11` reader"]
pub type R = crate::R<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_11` writer"]
pub type W = crate::W<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0` reader - RTCSlow_0 memory split address in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0` writer - RTCSlow_0 memory split address in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1` reader - RTCSlow_0 memory split address in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1` writer - RTCSlow_0 memory split address in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - RTCSlow_0 memory split address in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_0(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTCSlow_0 memory split address in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_PIF_PMS_CONSTRAIN_11")
            .field(
                "core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_0",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_0()
                        .bits()
                ),
            )
            .field(
                "core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_1",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_1()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - RTCSlow_0 memory split address in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_W<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC, 0>
    {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_W::new(self)
    }
    #[doc = "Bits 11:21 - RTCSlow_0 memory split address in world 1 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_W<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC, 11>
    {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "core1 access peripherals permission configuration register 11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_11_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_pif_pms_constrain_11::R`](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_pif_pms_constrain_11::W`](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_11 to value 0x003f_ffff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_ffff;
}
