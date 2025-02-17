#[doc = "Register `FH1_STATUS` reader"]
pub type R = crate::R<FH1_STATUS_SPEC>;
#[doc = "Field `FH1_CBC_ON` reader - "]
pub type FH1_CBC_ON_R = crate::BitReader;
#[doc = "Field `FH1_OST_ON` reader - "]
pub type FH1_OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh1_cbc_on(&self) -> FH1_CBC_ON_R {
        FH1_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh1_ost_on(&self) -> FH1_OST_ON_R {
        FH1_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH1_STATUS")
            .field("fh1_cbc_on", &format_args!("{}", self.fh1_cbc_on().bit()))
            .field("fh1_ost_on", &format_args!("{}", self.fh1_ost_on().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH1_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH1_STATUS_SPEC;
impl crate::RegisterSpec for FH1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh1_status::R`](R) reader structure"]
impl crate::Readable for FH1_STATUS_SPEC {}
#[doc = "`reset()` method sets FH1_STATUS to value 0"]
impl crate::Resettable for FH1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
