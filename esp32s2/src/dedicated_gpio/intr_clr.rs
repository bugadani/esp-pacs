#[doc = "Register `INTR_CLR` writer"]
pub type W = crate::W<INTR_CLR_SPEC>;
#[doc = "Field `GPIO0_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO0_INT_RAW interrupt."]
pub type GPIO0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO1_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO1_INT_RAW interrupt."]
pub type GPIO1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO2_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO2_INT_RAW interrupt."]
pub type GPIO2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO3_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO3_INT_RAW interrupt."]
pub type GPIO3_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO4_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO4_INT_RAW interrupt."]
pub type GPIO4_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO5_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO5_INT_RAW interrupt."]
pub type GPIO5_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO6_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO6_INT_RAW interrupt."]
pub type GPIO6_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO7_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO7_INT_RAW interrupt."]
pub type GPIO7_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the DEDIC_GPIO0_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0_int_clr(&mut self) -> GPIO0_INT_CLR_W<INTR_CLR_SPEC, 0> {
        GPIO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the DEDIC_GPIO1_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1_int_clr(&mut self) -> GPIO1_INT_CLR_W<INTR_CLR_SPEC, 1> {
        GPIO1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the DEDIC_GPIO2_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2_int_clr(&mut self) -> GPIO2_INT_CLR_W<INTR_CLR_SPEC, 2> {
        GPIO2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the DEDIC_GPIO3_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio3_int_clr(&mut self) -> GPIO3_INT_CLR_W<INTR_CLR_SPEC, 3> {
        GPIO3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the DEDIC_GPIO4_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio4_int_clr(&mut self) -> GPIO4_INT_CLR_W<INTR_CLR_SPEC, 4> {
        GPIO4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the DEDIC_GPIO5_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio5_int_clr(&mut self) -> GPIO5_INT_CLR_W<INTR_CLR_SPEC, 5> {
        GPIO5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the DEDIC_GPIO6_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio6_int_clr(&mut self) -> GPIO6_INT_CLR_W<INTR_CLR_SPEC, 6> {
        GPIO6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the DEDIC_GPIO7_INT_RAW interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio7_int_clr(&mut self) -> GPIO7_INT_CLR_W<INTR_CLR_SPEC, 7> {
        GPIO7_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_CLR_SPEC;
impl crate::RegisterSpec for INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intr_clr::W`](W) writer structure"]
impl crate::Writable for INTR_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_CLR to value 0"]
impl crate::Resettable for INTR_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
