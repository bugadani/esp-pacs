#[doc = "Register `IN_DSCR_CH%s` reader"]
pub type R = crate::R<IN_DSCR_CH_SPEC>;
#[doc = "Field `INLINK_DSCR` reader - The address of the current inlink descriptor x."]
pub type INLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the current inlink descriptor x."]
    #[inline(always)]
    pub fn inlink_dscr(&self) -> INLINK_DSCR_R {
        INLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DSCR_CH")
            .field(
                "inlink_dscr",
                &format_args!("{}", self.inlink_dscr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_DSCR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Current inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_DSCR_CH_SPEC;
impl crate::RegisterSpec for IN_DSCR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dscr_ch::R`](R) reader structure"]
impl crate::Readable for IN_DSCR_CH_SPEC {}
#[doc = "`reset()` method sets IN_DSCR_CH%s to value 0"]
impl crate::Resettable for IN_DSCR_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
