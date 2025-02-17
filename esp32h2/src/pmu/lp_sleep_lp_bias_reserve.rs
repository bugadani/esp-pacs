#[doc = "Register `LP_SLEEP_LP_BIAS_RESERVE` writer"]
pub type W = crate::W<LP_SLEEP_LP_BIAS_RESERVE_SPEC>;
#[doc = "Field `LP_SLEEP_LP_BIAS_RESERVE` writer - need_des"]
pub type LP_SLEEP_LP_BIAS_RESERVE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_SLEEP_LP_BIAS_RESERVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_bias_reserve(
        &mut self,
    ) -> LP_SLEEP_LP_BIAS_RESERVE_W<LP_SLEEP_LP_BIAS_RESERVE_SPEC, 0> {
        LP_SLEEP_LP_BIAS_RESERVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_bias_reserve::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_SLEEP_LP_BIAS_RESERVE_SPEC;
impl crate::RegisterSpec for LP_SLEEP_LP_BIAS_RESERVE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_lp_bias_reserve::W`](W) writer structure"]
impl crate::Writable for LP_SLEEP_LP_BIAS_RESERVE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_BIAS_RESERVE to value 0"]
impl crate::Resettable for LP_SLEEP_LP_BIAS_RESERVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
