#[doc = "Register `RTC_GPIO_OUT` reader"]
pub type R = crate::R<RTC_GPIO_OUT_SPEC>;
#[doc = "Register `RTC_GPIO_OUT` writer"]
pub type W = crate::W<RTC_GPIO_OUT_SPEC>;
#[doc = "Field `DATA` reader - RTC GPIO 0 ~ 21 output data"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - RTC GPIO 0 ~ 21 output data"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_OUT")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<RTC_GPIO_OUT_SPEC, 10> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC GPIO 0 ~ 21 output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_OUT_SPEC;
impl crate::RegisterSpec for RTC_GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_out::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_out::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_OUT to value 0"]
impl crate::Resettable for RTC_GPIO_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
