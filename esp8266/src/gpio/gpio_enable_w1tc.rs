#[doc = "Register `GPIO_ENABLE_W1TC` writer"]
pub type W = crate::W<GPIO_ENABLE_W1TC_SPEC>;
#[doc = "Field `GPIO_ENABLE_DATA_W1TC` writer - Writing 1 into a bit in this register will clear the related bit in GPIO_ENABLE_DATA"]
pub type GPIO_ENABLE_DATA_W1TC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing 1 into a bit in this register will clear the related bit in GPIO_ENABLE_DATA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_enable_data_w1tc(&mut self) -> GPIO_ENABLE_DATA_W1TC_W<GPIO_ENABLE_W1TC_SPEC, 0> {
        GPIO_ENABLE_DATA_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO_ENABLE_W1TC\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_enable_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for GPIO_ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_enable_w1tc::W`](W) writer structure"]
impl crate::Writable for GPIO_ENABLE_W1TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_ENABLE_W1TC to value 0"]
impl crate::Resettable for GPIO_ENABLE_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
