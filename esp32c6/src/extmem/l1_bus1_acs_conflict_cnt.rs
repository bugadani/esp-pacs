#[doc = "Register `L1_BUS1_ACS_CONFLICT_CNT` reader"]
pub struct R(crate::R<L1_BUS1_ACS_CONFLICT_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_BUS1_ACS_CONFLICT_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_BUS1_ACS_CONFLICT_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_BUS1_ACS_CONFLICT_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_BUS1_CONFLICT_CNT` reader - The register records the number of access-conflicts when bus1 accesses L1-Cache."]
pub type L1_BUS1_CONFLICT_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of access-conflicts when bus1 accesses L1-Cache."]
    #[inline(always)]
    pub fn l1_bus1_conflict_cnt(&self) -> L1_BUS1_CONFLICT_CNT_R {
        L1_BUS1_CONFLICT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_BUS1_ACS_CONFLICT_CNT")
            .field(
                "l1_bus1_conflict_cnt",
                &format_args!("{}", self.l1_bus1_conflict_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_BUS1_ACS_CONFLICT_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1-Cache bus1 Conflict-Access Counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_bus1_acs_conflict_cnt](index.html) module"]
pub struct L1_BUS1_ACS_CONFLICT_CNT_SPEC;
impl crate::RegisterSpec for L1_BUS1_ACS_CONFLICT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_bus1_acs_conflict_cnt::R](R) reader structure"]
impl crate::Readable for L1_BUS1_ACS_CONFLICT_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_BUS1_ACS_CONFLICT_CNT to value 0"]
impl crate::Resettable for L1_BUS1_ACS_CONFLICT_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
