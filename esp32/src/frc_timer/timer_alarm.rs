#[doc = "Register `TIMER_ALARM` reader"]
pub type R = crate::R<TIMER_ALARM_SPEC>;
#[doc = "Register `TIMER_ALARM` writer"]
pub type W = crate::W<TIMER_ALARM_SPEC>;
#[doc = "Field `TIMER_ALARM` reader - "]
pub type TIMER_ALARM_R = crate::FieldReader;
#[doc = "Field `TIMER_ALARM` writer - "]
pub type TIMER_ALARM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_alarm(&self) -> TIMER_ALARM_R {
        TIMER_ALARM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_ALARM")
            .field(
                "timer_alarm",
                &format_args!("{}", self.timer_alarm().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_ALARM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn timer_alarm(&mut self) -> TIMER_ALARM_W<TIMER_ALARM_SPEC, 0> {
        TIMER_ALARM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_alarm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_alarm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_ALARM_SPEC;
impl crate::RegisterSpec for TIMER_ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_alarm::R`](R) reader structure"]
impl crate::Readable for TIMER_ALARM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_alarm::W`](W) writer structure"]
impl crate::Writable for TIMER_ALARM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_ALARM to value 0"]
impl crate::Resettable for TIMER_ALARM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
