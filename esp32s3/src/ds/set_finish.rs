#[doc = "Register `SET_FINISH` writer"]
pub type W = crate::W<SET_FINISH_SPEC>;
#[doc = "Field `SET_FINISH` writer - Write 1 to this register to end DS operation."]
pub type SET_FINISH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_FINISH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this register to end DS operation."]
    #[inline(always)]
    #[must_use]
    pub fn set_finish(&mut self) -> SET_FINISH_W<SET_FINISH_SPEC, 0> {
        SET_FINISH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ends DS operation\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_finish::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_FINISH_SPEC;
impl crate::RegisterSpec for SET_FINISH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_finish::W`](W) writer structure"]
impl crate::Writable for SET_FINISH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_FINISH to value 0"]
impl crate::Resettable for SET_FINISH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
