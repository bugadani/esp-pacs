#[doc = "Register `STATUS_NEXT1` reader"]
pub type R = crate::R<STATUS_NEXT1_SPEC>;
#[doc = "Field `STATUS1_INTERRUPT_NEXT` reader - Interrupt source signal of GPIO32 ~ 53."]
pub type STATUS1_INTERRUPT_NEXT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Interrupt source signal of GPIO32 ~ 53."]
    #[inline(always)]
    pub fn status1_interrupt_next(&self) -> STATUS1_INTERRUPT_NEXT_R {
        STATUS1_INTERRUPT_NEXT_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_NEXT1")
            .field(
                "status1_interrupt_next",
                &format_args!("{}", self.status1_interrupt_next().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_NEXT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO32 ~ 53 interrupt source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_NEXT1_SPEC;
impl crate::RegisterSpec for STATUS_NEXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_next1::R`](R) reader structure"]
impl crate::Readable for STATUS_NEXT1_SPEC {}
#[doc = "`reset()` method sets STATUS_NEXT1 to value 0"]
impl crate::Resettable for STATUS_NEXT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
