#[doc = "Register `LSTIMER%s_VALUE` reader"]
pub type R = crate::R<LSTIMER_VALUE_SPEC>;
#[doc = "Field `CNT` reader - software can read this register to get the current counter value in low speed timer0."]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer0."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSTIMER_VALUE")
            .field("cnt", &format_args!("{}", self.cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LSTIMER_VALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstimer_value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSTIMER_VALUE_SPEC;
impl crate::RegisterSpec for LSTIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lstimer_value::R`](R) reader structure"]
impl crate::Readable for LSTIMER_VALUE_SPEC {}
#[doc = "`reset()` method sets LSTIMER%s_VALUE to value 0"]
impl crate::Resettable for LSTIMER_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
