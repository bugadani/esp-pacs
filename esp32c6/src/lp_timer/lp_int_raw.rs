#[doc = "Register `LP_INT_RAW` reader"]
pub type R = crate::R<LP_INT_RAW_SPEC>;
#[doc = "Register `LP_INT_RAW` writer"]
pub type W = crate::W<LP_INT_RAW_SPEC>;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_RAW` reader - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_RAW_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_RAW` writer - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAIN_TIMER_LP_INT_RAW` reader - need_des"]
pub type MAIN_TIMER_LP_INT_RAW_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_LP_INT_RAW` writer - need_des"]
pub type MAIN_TIMER_LP_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_raw(&self) -> MAIN_TIMER_OVERFLOW_LP_INT_RAW_R {
        MAIN_TIMER_OVERFLOW_LP_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_lp_int_raw(&self) -> MAIN_TIMER_LP_INT_RAW_R {
        MAIN_TIMER_LP_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_RAW")
            .field(
                "main_timer_overflow_lp_int_raw",
                &format_args!("{}", self.main_timer_overflow_lp_int_raw().bit()),
            )
            .field(
                "main_timer_lp_int_raw",
                &format_args!("{}", self.main_timer_lp_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_overflow_lp_int_raw(
        &mut self,
    ) -> MAIN_TIMER_OVERFLOW_LP_INT_RAW_W<LP_INT_RAW_SPEC, 30> {
        MAIN_TIMER_OVERFLOW_LP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_lp_int_raw(&mut self) -> MAIN_TIMER_LP_INT_RAW_W<LP_INT_RAW_SPEC, 31> {
        MAIN_TIMER_LP_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_raw::R`](R) reader structure"]
impl crate::Readable for LP_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_int_raw::W`](W) writer structure"]
impl crate::Writable for LP_INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_INT_RAW to value 0"]
impl crate::Resettable for LP_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
