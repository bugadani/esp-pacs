#[doc = "Register `TARGET2_CONF` reader"]
pub type R = crate::R<TARGET2_CONF_SPEC>;
#[doc = "Register `TARGET2_CONF` writer"]
pub type W = crate::W<TARGET2_CONF_SPEC>;
#[doc = "Field `TARGET2_PERIOD` reader - target2 period"]
pub type TARGET2_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET2_PERIOD` writer - target2 period"]
pub type TARGET2_PERIOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
#[doc = "Field `TARGET2_PERIOD_MODE` reader - Set target2 to period mode"]
pub type TARGET2_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET2_PERIOD_MODE` writer - Set target2 to period mode"]
pub type TARGET2_PERIOD_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET2_TIMER_UNIT_SEL` reader - select which unit to compare"]
pub type TARGET2_TIMER_UNIT_SEL_R = crate::BitReader;
#[doc = "Field `TARGET2_TIMER_UNIT_SEL` writer - select which unit to compare"]
pub type TARGET2_TIMER_UNIT_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:25 - target2 period"]
    #[inline(always)]
    pub fn target2_period(&self) -> TARGET2_PERIOD_R {
        TARGET2_PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Set target2 to period mode"]
    #[inline(always)]
    pub fn target2_period_mode(&self) -> TARGET2_PERIOD_MODE_R {
        TARGET2_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    pub fn target2_timer_unit_sel(&self) -> TARGET2_TIMER_UNIT_SEL_R {
        TARGET2_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET2_CONF")
            .field(
                "target2_period",
                &format_args!("{}", self.target2_period().bits()),
            )
            .field(
                "target2_period_mode",
                &format_args!("{}", self.target2_period_mode().bit()),
            )
            .field(
                "target2_timer_unit_sel",
                &format_args!("{}", self.target2_timer_unit_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET2_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:25 - target2 period"]
    #[inline(always)]
    #[must_use]
    pub fn target2_period(&mut self) -> TARGET2_PERIOD_W<TARGET2_CONF_SPEC, 0> {
        TARGET2_PERIOD_W::new(self)
    }
    #[doc = "Bit 30 - Set target2 to period mode"]
    #[inline(always)]
    #[must_use]
    pub fn target2_period_mode(&mut self) -> TARGET2_PERIOD_MODE_W<TARGET2_CONF_SPEC, 30> {
        TARGET2_PERIOD_MODE_W::new(self)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    #[must_use]
    pub fn target2_timer_unit_sel(&mut self) -> TARGET2_TIMER_UNIT_SEL_W<TARGET2_CONF_SPEC, 31> {
        TARGET2_TIMER_UNIT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "system timer comp2 target mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target2_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target2_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET2_CONF_SPEC;
impl crate::RegisterSpec for TARGET2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target2_conf::R`](R) reader structure"]
impl crate::Readable for TARGET2_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target2_conf::W`](W) writer structure"]
impl crate::Writable for TARGET2_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET2_CONF to value 0"]
impl crate::Resettable for TARGET2_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
