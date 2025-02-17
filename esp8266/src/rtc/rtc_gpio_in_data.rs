#[doc = "Register `RTC_GPIO_IN_DATA` reader"]
pub type R = crate::R<RTC_GPIO_IN_DATA_SPEC>;
#[doc = "Register `RTC_GPIO_IN_DATA` writer"]
pub type W = crate::W<RTC_GPIO_IN_DATA_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_IN_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC_GPIO_IN_DATA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_in_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_in_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_IN_DATA_SPEC;
impl crate::RegisterSpec for RTC_GPIO_IN_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_in_data::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_IN_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_in_data::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_IN_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_IN_DATA to value 0"]
impl crate::Resettable for RTC_GPIO_IN_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
