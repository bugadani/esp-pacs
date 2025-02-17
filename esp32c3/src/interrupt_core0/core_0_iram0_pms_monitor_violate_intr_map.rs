#[doc = "Register `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` reader"]
pub type R = crate::R<CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
#[doc = "Register `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` writer"]
pub type W = crate::W<CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` reader - reg_core0_core_0_iram0_pms_monitor_violate_intr_map"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` writer - reg_core0_core_0_iram0_pms_monitor_violate_intr_map"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_core_0_iram0_pms_monitor_violate_intr_map"]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_intr_map(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP")
            .field(
                "core_0_iram0_pms_monitor_violate_intr_map",
                &format_args!(
                    "{}",
                    self.core_0_iram0_pms_monitor_violate_intr_map().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_core_0_iram0_pms_monitor_violate_intr_map"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_iram0_pms_monitor_violate_intr_map(
        &mut self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W<
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
        0,
    > {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "iram0 pms violatile intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_pms_monitor_violate_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_iram0_pms_monitor_violate_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC;
impl crate::RegisterSpec for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_iram0_pms_monitor_violate_intr_map::R`](R) reader structure"]
impl crate::Readable for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_iram0_pms_monitor_violate_intr_map::W`](W) writer structure"]
impl crate::Writable for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP to value 0"]
impl crate::Resettable for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
