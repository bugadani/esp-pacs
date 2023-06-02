#[doc = "Register `L2_CACHE_ACS_FAIL_INT_ENA` reader"]
pub struct R(crate::R<L2_CACHE_ACS_FAIL_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_FAIL_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_FAIL_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_FAIL_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
pub type L2_CACHE_FAIL_INT_ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_int_ena(&self) -> L2_CACHE_FAIL_INT_ENA_R {
        L2_CACHE_FAIL_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_FAIL_INT_ENA")
            .field(
                "l2_cache_fail_int_ena",
                &format_args!("{}", self.l2_cache_fail_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACS_FAIL_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Access Fail Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_fail_int_ena](index.html) module"]
pub struct L2_CACHE_ACS_FAIL_INT_ENA_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_fail_int_ena::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_INT_ENA to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
