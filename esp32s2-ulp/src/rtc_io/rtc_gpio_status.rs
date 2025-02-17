#[doc = "Register `RTC_GPIO_STATUS` reader"]
pub type R = crate::R<RTC_GPIO_STATUS_SPEC>;
#[doc = "Register `RTC_GPIO_STATUS` writer"]
pub type W = crate::W<RTC_GPIO_STATUS_SPEC>;
#[doc = "Field `GPIO_STATUS_INT` reader - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
pub type GPIO_STATUS_INT_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_STATUS_INT` writer - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
pub type GPIO_STATUS_INT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
    #[inline(always)]
    pub fn gpio_status_int(&self) -> GPIO_STATUS_INT_R {
        GPIO_STATUS_INT_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_STATUS")
            .field(
                "gpio_status_int",
                &format_args!("{}", self.gpio_status_int().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_status_int(&mut self) -> GPIO_STATUS_INT_W<RTC_GPIO_STATUS_SPEC, 10> {
        GPIO_STATUS_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC GPIO interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_STATUS_SPEC;
impl crate::RegisterSpec for RTC_GPIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_status::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_status::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_STATUS to value 0"]
impl crate::Resettable for RTC_GPIO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
