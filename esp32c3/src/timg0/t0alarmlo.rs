#[doc = "Register `T0ALARMLO` reader"]
pub type R = crate::R<T0ALARMLO_SPEC>;
#[doc = "Register `T0ALARMLO` writer"]
pub type W = crate::W<T0ALARMLO_SPEC>;
#[doc = "Field `ALARM_LO` reader - reg_t0_alarm_lo."]
pub type ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_LO` writer - reg_t0_alarm_lo."]
pub type ALARM_LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_t0_alarm_lo."]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0ALARMLO")
            .field("alarm_lo", &format_args!("{}", self.alarm_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0ALARMLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_t0_alarm_lo."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W<T0ALARMLO_SPEC, 0> {
        ALARM_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TIMG_T0ALARMLO_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0ALARMLO_SPEC;
impl crate::RegisterSpec for T0ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0alarmlo::R`](R) reader structure"]
impl crate::Readable for T0ALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0alarmlo::W`](W) writer structure"]
impl crate::Writable for T0ALARMLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0ALARMLO to value 0"]
impl crate::Resettable for T0ALARMLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
