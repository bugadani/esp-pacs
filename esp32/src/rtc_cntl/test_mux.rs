#[doc = "Register `TEST_MUX` reader"]
pub type R = crate::R<TEST_MUX_SPEC>;
#[doc = "Register `TEST_MUX` writer"]
pub type W = crate::W<TEST_MUX_SPEC>;
#[doc = "Field `ENT_RTC` reader - ENT_RTC"]
pub type ENT_RTC_R = crate::BitReader;
#[doc = "Field `ENT_RTC` writer - ENT_RTC"]
pub type ENT_RTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTEST_RTC` reader - DTEST_RTC"]
pub type DTEST_RTC_R = crate::FieldReader;
#[doc = "Field `DTEST_RTC` writer - DTEST_RTC"]
pub type DTEST_RTC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn ent_rtc(&self) -> ENT_RTC_R {
        ENT_RTC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn dtest_rtc(&self) -> DTEST_RTC_R {
        DTEST_RTC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_MUX")
            .field("ent_rtc", &format_args!("{}", self.ent_rtc().bit()))
            .field("dtest_rtc", &format_args!("{}", self.dtest_rtc().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEST_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    #[must_use]
    pub fn ent_rtc(&mut self) -> ENT_RTC_W<TEST_MUX_SPEC, 29> {
        ENT_RTC_W::new(self)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_rtc(&mut self) -> DTEST_RTC_W<TEST_MUX_SPEC, 30> {
        DTEST_RTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test_mux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_mux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_MUX_SPEC;
impl crate::RegisterSpec for TEST_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_mux::R`](R) reader structure"]
impl crate::Readable for TEST_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test_mux::W`](W) writer structure"]
impl crate::Writable for TEST_MUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST_MUX to value 0"]
impl crate::Resettable for TEST_MUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
