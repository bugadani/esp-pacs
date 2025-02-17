#[doc = "Register `CORE_0_PIF_PMS_MONITOR_5` reader"]
pub type R = crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>;
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR` reader - Record core0 unsupported access type interrupt state."]
pub type CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R = crate::BitReader;
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE` reader - Record access type when core0 initiate unsupported access type."]
pub type CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD` reader - Record world information when core0 initiate unsupported access type."]
pub type CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Record core0 unsupported access type interrupt state."]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_intr(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Record access type when core0 initiate unsupported access type."]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_status_hsize(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Record world information when core0 initiate unsupported access type."]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_status_hworld(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_MONITOR_5")
            .field(
                "core_0_pif_pms_monitor_nonword_violate_intr",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_monitor_nonword_violate_intr().bit()
                ),
            )
            .field(
                "core_0_pif_pms_monitor_nonword_violate_status_hsize",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_monitor_nonword_violate_status_hsize()
                        .bits()
                ),
            )
            .field(
                "core_0_pif_pms_monitor_nonword_violate_status_hworld",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_monitor_nonword_violate_status_hworld()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_MONITOR_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Core0 permission report register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_PIF_PMS_MONITOR_5_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_pif_pms_monitor_5::R`](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_MONITOR_5_SPEC {}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_MONITOR_5 to value 0"]
impl crate::Resettable for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
