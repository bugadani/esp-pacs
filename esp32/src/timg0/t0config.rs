#[doc = "Register `T0CONFIG` reader"]
pub type R = crate::R<T0CONFIG_SPEC>;
#[doc = "Register `T0CONFIG` writer"]
pub type W = crate::W<T0CONFIG_SPEC>;
#[doc = "Field `ALARM_EN` reader - When set alarm is enabled"]
pub type ALARM_EN_R = crate::BitReader;
#[doc = "Field `ALARM_EN` writer - When set alarm is enabled"]
pub type ALARM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEVEL_INT_EN` reader - When set level type interrupt will be generated during alarm"]
pub type LEVEL_INT_EN_R = crate::BitReader;
#[doc = "Field `LEVEL_INT_EN` writer - When set level type interrupt will be generated during alarm"]
pub type LEVEL_INT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDGE_INT_EN` reader - When set edge type interrupt will be generated during alarm"]
pub type EDGE_INT_EN_R = crate::BitReader;
#[doc = "Field `EDGE_INT_EN` writer - When set edge type interrupt will be generated during alarm"]
pub type EDGE_INT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVIDER` reader - Timer 0 clock (T0_clk) prescale value."]
pub type DIVIDER_R = crate::FieldReader<u16>;
#[doc = "Field `DIVIDER` writer - Timer 0 clock (T0_clk) prescale value."]
pub type DIVIDER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `AUTORELOAD` reader - When set timer 0 auto-reload at alarming is enabled"]
pub type AUTORELOAD_R = crate::BitReader;
#[doc = "Field `AUTORELOAD` writer - When set timer 0 auto-reload at alarming is enabled"]
pub type AUTORELOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCREASE` reader - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
pub type INCREASE_R = crate::BitReader;
#[doc = "Field `INCREASE` writer - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
pub type INCREASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - When set timer 0 time-base counter is enabled"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - When set timer 0 time-base counter is enabled"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn level_int_en(&self) -> LEVEL_INT_EN_R {
        LEVEL_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn edge_int_en(&self) -> EDGE_INT_EN_R {
        EDGE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Timer 0 clock (T0_clk) prescale value."]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - When set timer 0 auto-reload at alarming is enabled"]
    #[inline(always)]
    pub fn autoreload(&self) -> AUTORELOAD_R {
        AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set timer 0 time-base counter is enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0CONFIG")
            .field("alarm_en", &format_args!("{}", self.alarm_en().bit()))
            .field(
                "level_int_en",
                &format_args!("{}", self.level_int_en().bit()),
            )
            .field("edge_int_en", &format_args!("{}", self.edge_int_en().bit()))
            .field("divider", &format_args!("{}", self.divider().bits()))
            .field("autoreload", &format_args!("{}", self.autoreload().bit()))
            .field("increase", &format_args!("{}", self.increase().bit()))
            .field("en", &format_args!("{}", self.en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<T0CONFIG_SPEC, 10> {
        ALARM_EN_W::new(self)
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    #[must_use]
    pub fn level_int_en(&mut self) -> LEVEL_INT_EN_W<T0CONFIG_SPEC, 11> {
        LEVEL_INT_EN_W::new(self)
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    #[must_use]
    pub fn edge_int_en(&mut self) -> EDGE_INT_EN_W<T0CONFIG_SPEC, 12> {
        EDGE_INT_EN_W::new(self)
    }
    #[doc = "Bits 13:28 - Timer 0 clock (T0_clk) prescale value."]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<T0CONFIG_SPEC, 13> {
        DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - When set timer 0 auto-reload at alarming is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<T0CONFIG_SPEC, 29> {
        AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
    #[inline(always)]
    #[must_use]
    pub fn increase(&mut self) -> INCREASE_W<T0CONFIG_SPEC, 30> {
        INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - When set timer 0 time-base counter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<T0CONFIG_SPEC, 31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0CONFIG_SPEC;
impl crate::RegisterSpec for T0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0config::R`](R) reader structure"]
impl crate::Readable for T0CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0config::W`](W) writer structure"]
impl crate::Writable for T0CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0CONFIG to value 0x6000_2000"]
impl crate::Resettable for T0CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_2000;
}
