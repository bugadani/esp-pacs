#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `SYSTIMER_CLK_FO` reader - systimer clock force on"]
pub type SYSTIMER_CLK_FO_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_FO` writer - systimer clock force on"]
pub type SYSTIMER_CLK_FO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET2_WORK_EN` reader - target2 work enable"]
pub type TARGET2_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET2_WORK_EN` writer - target2 work enable"]
pub type TARGET2_WORK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET1_WORK_EN` reader - target1 work enable"]
pub type TARGET1_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET1_WORK_EN` writer - target1 work enable"]
pub type TARGET1_WORK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET0_WORK_EN` reader - target0 work enable"]
pub type TARGET0_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET0_WORK_EN` writer - target0 work enable"]
pub type TARGET0_WORK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` reader - If timer unit1 is stalled when core1 stalled"]
pub type TIMER_UNIT1_CORE1_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` writer - If timer unit1 is stalled when core1 stalled"]
pub type TIMER_UNIT1_CORE1_STALL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` reader - If timer unit1 is stalled when core0 stalled"]
pub type TIMER_UNIT1_CORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` writer - If timer unit1 is stalled when core0 stalled"]
pub type TIMER_UNIT1_CORE0_STALL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` reader - If timer unit0 is stalled when core1 stalled"]
pub type TIMER_UNIT0_CORE1_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` writer - If timer unit0 is stalled when core1 stalled"]
pub type TIMER_UNIT0_CORE1_STALL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` reader - If timer unit0 is stalled when core0 stalled"]
pub type TIMER_UNIT0_CORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` writer - If timer unit0 is stalled when core0 stalled"]
pub type TIMER_UNIT0_CORE0_STALL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_UNIT1_WORK_EN` reader - timer unit1 work enable"]
pub type TIMER_UNIT1_WORK_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_WORK_EN` writer - timer unit1 work enable"]
pub type TIMER_UNIT1_WORK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_UNIT0_WORK_EN` reader - timer unit0 work enable"]
pub type TIMER_UNIT0_WORK_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_WORK_EN` writer - timer unit0 work enable"]
pub type TIMER_UNIT0_WORK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_EN` reader - register file clk gating"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - register file clk gating"]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&self) -> SYSTIMER_CLK_FO_R {
        SYSTIMER_CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    pub fn target2_work_en(&self) -> TARGET2_WORK_EN_R {
        TARGET2_WORK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    pub fn target1_work_en(&self) -> TARGET1_WORK_EN_R {
        TARGET1_WORK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    pub fn target0_work_en(&self) -> TARGET0_WORK_EN_R {
        TARGET0_WORK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&self) -> TIMER_UNIT1_CORE1_STALL_EN_R {
        TIMER_UNIT1_CORE1_STALL_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&self) -> TIMER_UNIT1_CORE0_STALL_EN_R {
        TIMER_UNIT1_CORE0_STALL_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&self) -> TIMER_UNIT0_CORE1_STALL_EN_R {
        TIMER_UNIT0_CORE1_STALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&self) -> TIMER_UNIT0_CORE0_STALL_EN_R {
        TIMER_UNIT0_CORE0_STALL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&self) -> TIMER_UNIT1_WORK_EN_R {
        TIMER_UNIT1_WORK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&self) -> TIMER_UNIT0_WORK_EN_R {
        TIMER_UNIT0_WORK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field(
                "systimer_clk_fo",
                &format_args!("{}", self.systimer_clk_fo().bit()),
            )
            .field(
                "target2_work_en",
                &format_args!("{}", self.target2_work_en().bit()),
            )
            .field(
                "target1_work_en",
                &format_args!("{}", self.target1_work_en().bit()),
            )
            .field(
                "target0_work_en",
                &format_args!("{}", self.target0_work_en().bit()),
            )
            .field(
                "timer_unit1_core1_stall_en",
                &format_args!("{}", self.timer_unit1_core1_stall_en().bit()),
            )
            .field(
                "timer_unit1_core0_stall_en",
                &format_args!("{}", self.timer_unit1_core0_stall_en().bit()),
            )
            .field(
                "timer_unit0_core1_stall_en",
                &format_args!("{}", self.timer_unit0_core1_stall_en().bit()),
            )
            .field(
                "timer_unit0_core0_stall_en",
                &format_args!("{}", self.timer_unit0_core0_stall_en().bit()),
            )
            .field(
                "timer_unit1_work_en",
                &format_args!("{}", self.timer_unit1_work_en().bit()),
            )
            .field(
                "timer_unit0_work_en",
                &format_args!("{}", self.timer_unit0_work_en().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_fo(&mut self) -> SYSTIMER_CLK_FO_W<CONF_SPEC, 0> {
        SYSTIMER_CLK_FO_W::new(self)
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    #[must_use]
    pub fn target2_work_en(&mut self) -> TARGET2_WORK_EN_W<CONF_SPEC, 22> {
        TARGET2_WORK_EN_W::new(self)
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    #[must_use]
    pub fn target1_work_en(&mut self) -> TARGET1_WORK_EN_W<CONF_SPEC, 23> {
        TARGET1_WORK_EN_W::new(self)
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    #[must_use]
    pub fn target0_work_en(&mut self) -> TARGET0_WORK_EN_W<CONF_SPEC, 24> {
        TARGET0_WORK_EN_W::new(self)
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_core1_stall_en(&mut self) -> TIMER_UNIT1_CORE1_STALL_EN_W<CONF_SPEC, 25> {
        TIMER_UNIT1_CORE1_STALL_EN_W::new(self)
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_core0_stall_en(&mut self) -> TIMER_UNIT1_CORE0_STALL_EN_W<CONF_SPEC, 26> {
        TIMER_UNIT1_CORE0_STALL_EN_W::new(self)
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_core1_stall_en(&mut self) -> TIMER_UNIT0_CORE1_STALL_EN_W<CONF_SPEC, 27> {
        TIMER_UNIT0_CORE1_STALL_EN_W::new(self)
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_core0_stall_en(&mut self) -> TIMER_UNIT0_CORE0_STALL_EN_W<CONF_SPEC, 28> {
        TIMER_UNIT0_CORE0_STALL_EN_W::new(self)
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_work_en(&mut self) -> TIMER_UNIT1_WORK_EN_W<CONF_SPEC, 29> {
        TIMER_UNIT1_WORK_EN_W::new(self)
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_work_en(&mut self) -> TIMER_UNIT0_WORK_EN_W<CONF_SPEC, 30> {
        TIMER_UNIT0_WORK_EN_W::new(self)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC, 31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSTIMER_CONF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF to value 0x4600_0000"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x4600_0000;
}
