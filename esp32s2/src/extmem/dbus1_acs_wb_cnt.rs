#[doc = "Register `DBUS1_ACS_WB_CNT` reader"]
pub type R = crate::R<DBUS1_ACS_WB_CNT_SPEC>;
#[doc = "Field `DBUS1_ACS_WB_CNT` reader - The bits are used to count the number of cache evictions by dbus1 access cache."]
pub type DBUS1_ACS_WB_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - The bits are used to count the number of cache evictions by dbus1 access cache."]
    #[inline(always)]
    pub fn dbus1_acs_wb_cnt(&self) -> DBUS1_ACS_WB_CNT_R {
        DBUS1_ACS_WB_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS1_ACS_WB_CNT")
            .field(
                "dbus1_acs_wb_cnt",
                &format_args!("{}", self.dbus1_acs_wb_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS1_ACS_WB_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus1_acs_wb_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS1_ACS_WB_CNT_SPEC;
impl crate::RegisterSpec for DBUS1_ACS_WB_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus1_acs_wb_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS1_ACS_WB_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS1_ACS_WB_CNT to value 0"]
impl crate::Resettable for DBUS1_ACS_WB_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
