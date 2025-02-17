#[doc = "Register `REGCLK` reader"]
pub type R = crate::R<REGCLK_SPEC>;
#[doc = "Register `REGCLK` writer"]
pub type W = crate::W<REGCLK_SPEC>;
#[doc = "Field `ETM_EN` reader - enable timer's etm task and event"]
pub type ETM_EN_R = crate::BitReader;
#[doc = "Field `ETM_EN` writer - enable timer's etm task and event"]
pub type ETM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_CLK_IS_ACTIVE` reader - enable WDT's clock"]
pub type WDT_CLK_IS_ACTIVE_R = crate::BitReader;
#[doc = "Field `WDT_CLK_IS_ACTIVE` writer - enable WDT's clock"]
pub type WDT_CLK_IS_ACTIVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_CLK_IS_ACTIVE` reader - enable Timer 30's clock"]
pub type TIMER_CLK_IS_ACTIVE_R = crate::BitReader;
#[doc = "Field `TIMER_CLK_IS_ACTIVE` writer - enable Timer 30's clock"]
pub type TIMER_CLK_IS_ACTIVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_EN` reader - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 28 - enable timer's etm task and event"]
    #[inline(always)]
    pub fn etm_en(&self) -> ETM_EN_R {
        ETM_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable WDT's clock"]
    #[inline(always)]
    pub fn wdt_clk_is_active(&self) -> WDT_CLK_IS_ACTIVE_R {
        WDT_CLK_IS_ACTIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable Timer 30's clock"]
    #[inline(always)]
    pub fn timer_clk_is_active(&self) -> TIMER_CLK_IS_ACTIVE_R {
        TIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGCLK")
            .field("etm_en", &format_args!("{}", self.etm_en().bit()))
            .field(
                "wdt_clk_is_active",
                &format_args!("{}", self.wdt_clk_is_active().bit()),
            )
            .field(
                "timer_clk_is_active",
                &format_args!("{}", self.timer_clk_is_active().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - enable timer's etm task and event"]
    #[inline(always)]
    #[must_use]
    pub fn etm_en(&mut self) -> ETM_EN_W<REGCLK_SPEC, 28> {
        ETM_EN_W::new(self)
    }
    #[doc = "Bit 29 - enable WDT's clock"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_clk_is_active(&mut self) -> WDT_CLK_IS_ACTIVE_W<REGCLK_SPEC, 29> {
        WDT_CLK_IS_ACTIVE_W::new(self)
    }
    #[doc = "Bit 30 - enable Timer 30's clock"]
    #[inline(always)]
    #[must_use]
    pub fn timer_clk_is_active(&mut self) -> TIMER_CLK_IS_ACTIVE_W<REGCLK_SPEC, 30> {
        TIMER_CLK_IS_ACTIVE_W::new(self)
    }
    #[doc = "Bit 31 - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<REGCLK_SPEC, 31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer group clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGCLK_SPEC;
impl crate::RegisterSpec for REGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regclk::R`](R) reader structure"]
impl crate::Readable for REGCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regclk::W`](W) writer structure"]
impl crate::Writable for REGCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGCLK to value 0x7000_0000"]
impl crate::Resettable for REGCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000_0000;
}
