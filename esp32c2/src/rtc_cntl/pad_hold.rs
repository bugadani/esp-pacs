#[doc = "Register `PAD_HOLD` reader"]
pub type R = crate::R<PAD_HOLD_SPEC>;
#[doc = "Register `PAD_HOLD` writer"]
pub type W = crate::W<PAD_HOLD_SPEC>;
#[doc = "Field `GPIO_PIN0_HOLD` reader - Need add desc"]
pub type GPIO_PIN0_HOLD_R = crate::BitReader;
#[doc = "Field `GPIO_PIN0_HOLD` writer - Need add desc"]
pub type GPIO_PIN0_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN1_HOLD` reader - Need add desc"]
pub type GPIO_PIN1_HOLD_R = crate::BitReader;
#[doc = "Field `GPIO_PIN1_HOLD` writer - Need add desc"]
pub type GPIO_PIN1_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN2_HOLD` reader - Need add desc"]
pub type GPIO_PIN2_HOLD_R = crate::BitReader;
#[doc = "Field `GPIO_PIN2_HOLD` writer - Need add desc"]
pub type GPIO_PIN2_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN3_HOLD` reader - Need add desc"]
pub type GPIO_PIN3_HOLD_R = crate::BitReader;
#[doc = "Field `GPIO_PIN3_HOLD` writer - Need add desc"]
pub type GPIO_PIN3_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN4_HOLD` reader - Need add desc"]
pub type GPIO_PIN4_HOLD_R = crate::BitReader;
#[doc = "Field `GPIO_PIN4_HOLD` writer - Need add desc"]
pub type GPIO_PIN4_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN5_HOLD` reader - Need add desc"]
pub type GPIO_PIN5_HOLD_R = crate::BitReader;
#[doc = "Field `GPIO_PIN5_HOLD` writer - Need add desc"]
pub type GPIO_PIN5_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_hold(&self) -> GPIO_PIN0_HOLD_R {
        GPIO_PIN0_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_hold(&self) -> GPIO_PIN1_HOLD_R {
        GPIO_PIN1_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_hold(&self) -> GPIO_PIN2_HOLD_R {
        GPIO_PIN2_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_hold(&self) -> GPIO_PIN3_HOLD_R {
        GPIO_PIN3_HOLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_hold(&self) -> GPIO_PIN4_HOLD_R {
        GPIO_PIN4_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_hold(&self) -> GPIO_PIN5_HOLD_R {
        GPIO_PIN5_HOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_HOLD")
            .field(
                "gpio_pin0_hold",
                &format_args!("{}", self.gpio_pin0_hold().bit()),
            )
            .field(
                "gpio_pin1_hold",
                &format_args!("{}", self.gpio_pin1_hold().bit()),
            )
            .field(
                "gpio_pin2_hold",
                &format_args!("{}", self.gpio_pin2_hold().bit()),
            )
            .field(
                "gpio_pin3_hold",
                &format_args!("{}", self.gpio_pin3_hold().bit()),
            )
            .field(
                "gpio_pin4_hold",
                &format_args!("{}", self.gpio_pin4_hold().bit()),
            )
            .field(
                "gpio_pin5_hold",
                &format_args!("{}", self.gpio_pin5_hold().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD_HOLD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_hold(&mut self) -> GPIO_PIN0_HOLD_W<PAD_HOLD_SPEC, 0> {
        GPIO_PIN0_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_hold(&mut self) -> GPIO_PIN1_HOLD_W<PAD_HOLD_SPEC, 1> {
        GPIO_PIN1_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_hold(&mut self) -> GPIO_PIN2_HOLD_W<PAD_HOLD_SPEC, 2> {
        GPIO_PIN2_HOLD_W::new(self)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_hold(&mut self) -> GPIO_PIN3_HOLD_W<PAD_HOLD_SPEC, 3> {
        GPIO_PIN3_HOLD_W::new(self)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_hold(&mut self) -> GPIO_PIN4_HOLD_W<PAD_HOLD_SPEC, 4> {
        GPIO_PIN4_HOLD_W::new(self)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_hold(&mut self) -> GPIO_PIN5_HOLD_W<PAD_HOLD_SPEC, 5> {
        GPIO_PIN5_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_HOLD_SPEC;
impl crate::RegisterSpec for PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_hold::R`](R) reader structure"]
impl crate::Readable for PAD_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_hold::W`](W) writer structure"]
impl crate::Writable for PAD_HOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_HOLD to value 0"]
impl crate::Resettable for PAD_HOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
