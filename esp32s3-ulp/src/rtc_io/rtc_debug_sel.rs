#[doc = "Register `RTC_DEBUG_SEL` reader"]
pub type R = crate::R<RTC_DEBUG_SEL_SPEC>;
#[doc = "Register `RTC_DEBUG_SEL` writer"]
pub type W = crate::W<RTC_DEBUG_SEL_SPEC>;
#[doc = "Field `RTC_DEBUG_SEL0` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL0_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL0` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL1` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL1_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL1` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL2` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL2_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL2` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL3` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL3_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL3` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL4` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL4_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL4` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` reader - configure rtc debug"]
pub type RTC_DEBUG_12M_NO_GATING_R = crate::BitReader;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` writer - configure rtc debug"]
pub type RTC_DEBUG_12M_NO_GATING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&self) -> RTC_DEBUG_SEL0_R {
        RTC_DEBUG_SEL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&self) -> RTC_DEBUG_SEL1_R {
        RTC_DEBUG_SEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&self) -> RTC_DEBUG_SEL2_R {
        RTC_DEBUG_SEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&self) -> RTC_DEBUG_SEL3_R {
        RTC_DEBUG_SEL3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&self) -> RTC_DEBUG_SEL4_R {
        RTC_DEBUG_SEL4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&self) -> RTC_DEBUG_12M_NO_GATING_R {
        RTC_DEBUG_12M_NO_GATING_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_DEBUG_SEL")
            .field(
                "rtc_debug_sel0",
                &format_args!("{}", self.rtc_debug_sel0().bits()),
            )
            .field(
                "rtc_debug_sel1",
                &format_args!("{}", self.rtc_debug_sel1().bits()),
            )
            .field(
                "rtc_debug_sel2",
                &format_args!("{}", self.rtc_debug_sel2().bits()),
            )
            .field(
                "rtc_debug_sel3",
                &format_args!("{}", self.rtc_debug_sel3().bits()),
            )
            .field(
                "rtc_debug_sel4",
                &format_args!("{}", self.rtc_debug_sel4().bits()),
            )
            .field(
                "rtc_debug_12m_no_gating",
                &format_args!("{}", self.rtc_debug_12m_no_gating().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_DEBUG_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel0(&mut self) -> RTC_DEBUG_SEL0_W<RTC_DEBUG_SEL_SPEC, 0> {
        RTC_DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 5:9 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel1(&mut self) -> RTC_DEBUG_SEL1_W<RTC_DEBUG_SEL_SPEC, 5> {
        RTC_DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 10:14 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel2(&mut self) -> RTC_DEBUG_SEL2_W<RTC_DEBUG_SEL_SPEC, 10> {
        RTC_DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 15:19 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel3(&mut self) -> RTC_DEBUG_SEL3_W<RTC_DEBUG_SEL_SPEC, 15> {
        RTC_DEBUG_SEL3_W::new(self)
    }
    #[doc = "Bits 20:24 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel4(&mut self) -> RTC_DEBUG_SEL4_W<RTC_DEBUG_SEL_SPEC, 20> {
        RTC_DEBUG_SEL4_W::new(self)
    }
    #[doc = "Bit 25 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_12m_no_gating(&mut self) -> RTC_DEBUG_12M_NO_GATING_W<RTC_DEBUG_SEL_SPEC, 25> {
        RTC_DEBUG_12M_NO_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure rtc debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_debug_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_debug_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_DEBUG_SEL_SPEC;
impl crate::RegisterSpec for RTC_DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_debug_sel::R`](R) reader structure"]
impl crate::Readable for RTC_DEBUG_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_debug_sel::W`](W) writer structure"]
impl crate::Writable for RTC_DEBUG_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_DEBUG_SEL to value 0"]
impl crate::Resettable for RTC_DEBUG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
