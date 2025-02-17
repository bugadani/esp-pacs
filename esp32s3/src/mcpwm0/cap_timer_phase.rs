#[doc = "Register `CAP_TIMER_PHASE` reader"]
pub type R = crate::R<CAP_TIMER_PHASE_SPEC>;
#[doc = "Register `CAP_TIMER_PHASE` writer"]
pub type W = crate::W<CAP_TIMER_PHASE_SPEC>;
#[doc = "Field `CAP_PHASE` reader - Phase value for capture timer sync operation."]
pub type CAP_PHASE_R = crate::FieldReader<u32>;
#[doc = "Field `CAP_PHASE` writer - Phase value for capture timer sync operation."]
pub type CAP_PHASE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn cap_phase(&self) -> CAP_PHASE_R {
        CAP_PHASE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TIMER_PHASE")
            .field("cap_phase", &format_args!("{}", self.cap_phase().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_TIMER_PHASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    #[must_use]
    pub fn cap_phase(&mut self) -> CAP_PHASE_W<CAP_TIMER_PHASE_SPEC, 0> {
        CAP_PHASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Phase for capture timer sync\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_phase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_phase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_TIMER_PHASE_SPEC;
impl crate::RegisterSpec for CAP_TIMER_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_timer_phase::R`](R) reader structure"]
impl crate::Readable for CAP_TIMER_PHASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_timer_phase::W`](W) writer structure"]
impl crate::Writable for CAP_TIMER_PHASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_TIMER_PHASE to value 0"]
impl crate::Resettable for CAP_TIMER_PHASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
