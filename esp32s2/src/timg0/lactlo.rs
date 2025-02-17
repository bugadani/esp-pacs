#[doc = "Register `LACTLO` reader"]
pub type R = crate::R<LACTLO_SPEC>;
#[doc = "Field `LACT_LO` reader - Reserved."]
pub type LACT_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn lact_lo(&self) -> LACT_LO_R {
        LACT_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLO")
            .field("lact_lo", &format_args!("{}", self.lact_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "LACT low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactlo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTLO_SPEC;
impl crate::RegisterSpec for LACTLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactlo::R`](R) reader structure"]
impl crate::Readable for LACTLO_SPEC {}
#[doc = "`reset()` method sets LACTLO to value 0"]
impl crate::Resettable for LACTLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
