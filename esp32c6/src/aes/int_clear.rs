#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<INT_CLEAR_SPEC>;
#[doc = "Field `INT_CLEAR` writer - Set this bit to clear the AES interrupt."]
pub type INT_CLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the AES interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn int_clear(&mut self) -> INT_CLEAR_W<INT_CLEAR_SPEC, 0> {
        INT_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLEAR_SPEC;
impl crate::RegisterSpec for INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for INT_CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for INT_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
