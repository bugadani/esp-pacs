#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READ_DONE_INT_RAW` reader - read done interrupt raw status"]
pub type READ_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `PGM_DONE_INT_RAW` reader - program done interrupt raw status"]
pub type PGM_DONE_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - read done interrupt raw status"]
    #[inline(always)]
    pub fn read_done_int_raw(&self) -> READ_DONE_INT_RAW_R {
        READ_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program done interrupt raw status"]
    #[inline(always)]
    pub fn pgm_done_int_raw(&self) -> PGM_DONE_INT_RAW_R {
        PGM_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "read_done_int_raw",
                &format_args!("{}", self.read_done_int_raw().bit()),
            )
            .field(
                "pgm_done_int_raw",
                &format_args!("{}", self.pgm_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
