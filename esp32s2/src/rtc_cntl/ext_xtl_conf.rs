#[doc = "Register `EXT_XTL_CONF` reader"]
pub type R = crate::R<EXT_XTL_CONF_SPEC>;
#[doc = "Register `EXT_XTL_CONF` writer"]
pub type W = crate::W<EXT_XTL_CONF_SPEC>;
#[doc = "Field `XTAL32K_WDT_EN` reader - Set this bit to enable the 32 kHz crystal watchdog."]
pub type XTAL32K_WDT_EN_R = crate::BitReader;
#[doc = "Field `XTAL32K_WDT_EN` writer - Set this bit to enable the 32 kHz crystal watchdog."]
pub type XTAL32K_WDT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_WDT_CLK_FO` reader - Set this bit to FPU the 32 kHz crystal watchdog clock."]
pub type XTAL32K_WDT_CLK_FO_R = crate::BitReader;
#[doc = "Field `XTAL32K_WDT_CLK_FO` writer - Set this bit to FPU the 32 kHz crystal watchdog clock."]
pub type XTAL32K_WDT_CLK_FO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_WDT_RESET` reader - Set this bit to reset the 32 kHz crystal watchdog by SW."]
pub type XTAL32K_WDT_RESET_R = crate::BitReader;
#[doc = "Field `XTAL32K_WDT_RESET` writer - Set this bit to reset the 32 kHz crystal watchdog by SW."]
pub type XTAL32K_WDT_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_EXT_CLK_FO` reader - Set this bit to FPU the external clock of 32 kHz crystal."]
pub type XTAL32K_EXT_CLK_FO_R = crate::BitReader;
#[doc = "Field `XTAL32K_EXT_CLK_FO` writer - Set this bit to FPU the external clock of 32 kHz crystal."]
pub type XTAL32K_EXT_CLK_FO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_AUTO_BACKUP` reader - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_BACKUP_R = crate::BitReader;
#[doc = "Field `XTAL32K_AUTO_BACKUP` writer - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_BACKUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_AUTO_RESTART` reader - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_RESTART_R = crate::BitReader;
#[doc = "Field `XTAL32K_AUTO_RESTART` writer - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_RESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_AUTO_RETURN` reader - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
pub type XTAL32K_AUTO_RETURN_R = crate::BitReader;
#[doc = "Field `XTAL32K_AUTO_RETURN` writer - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
pub type XTAL32K_AUTO_RETURN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTAL32K_XPD_FORCE` reader - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
pub type XTAL32K_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `XTAL32K_XPD_FORCE` writer - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
pub type XTAL32K_XPD_FORCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENCKINIT_XTAL_32K` reader - Applies an internal clock to help the 32 kHz crystal to start."]
pub type ENCKINIT_XTAL_32K_R = crate::BitReader;
#[doc = "Field `ENCKINIT_XTAL_32K` writer - Applies an internal clock to help the 32 kHz crystal to start."]
pub type ENCKINIT_XTAL_32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBUF_XTAL_32K` reader - 0: single-end buffer 1: differential buffer"]
pub type DBUF_XTAL_32K_R = crate::BitReader;
#[doc = "Field `DBUF_XTAL_32K` writer - 0: single-end buffer 1: differential buffer"]
pub type DBUF_XTAL_32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DGM_XTAL_32K` reader - xtal_32k gm control"]
pub type DGM_XTAL_32K_R = crate::FieldReader;
#[doc = "Field `DGM_XTAL_32K` writer - xtal_32k gm control"]
pub type DGM_XTAL_32K_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DRES_XTAL_32K` reader - DRES_XTAL_32K"]
pub type DRES_XTAL_32K_R = crate::FieldReader;
#[doc = "Field `DRES_XTAL_32K` writer - DRES_XTAL_32K"]
pub type DRES_XTAL_32K_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `XPD_XTAL_32K` reader - XPD_XTAL_32K"]
pub type XPD_XTAL_32K_R = crate::BitReader;
#[doc = "Field `XPD_XTAL_32K` writer - XPD_XTAL_32K"]
pub type XPD_XTAL_32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAC_XTAL_32K` reader - DAC_XTAL_32K"]
pub type DAC_XTAL_32K_R = crate::FieldReader;
#[doc = "Field `DAC_XTAL_32K` writer - DAC_XTAL_32K"]
pub type DAC_XTAL_32K_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `WDT_STATE` reader - Stores the status of the 32 kHz watchdog."]
pub type WDT_STATE_R = crate::FieldReader;
#[doc = "Field `XTAL32K_GPIO_SEL` reader - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
pub type XTAL32K_GPIO_SEL_R = crate::BitReader;
#[doc = "Field `XTAL32K_GPIO_SEL` writer - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
pub type XTAL32K_GPIO_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTL_EXT_CTR_LV` reader - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
pub type XTL_EXT_CTR_LV_R = crate::BitReader;
#[doc = "Field `XTL_EXT_CTR_LV` writer - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
pub type XTL_EXT_CTR_LV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTL_EXT_CTR_EN` reader - Enables the GPIO to power down the crystal oscillator."]
pub type XTL_EXT_CTR_EN_R = crate::BitReader;
#[doc = "Field `XTL_EXT_CTR_EN` writer - Enables the GPIO to power down the crystal oscillator."]
pub type XTL_EXT_CTR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable the 32 kHz crystal watchdog."]
    #[inline(always)]
    pub fn xtal32k_wdt_en(&self) -> XTAL32K_WDT_EN_R {
        XTAL32K_WDT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to FPU the 32 kHz crystal watchdog clock."]
    #[inline(always)]
    pub fn xtal32k_wdt_clk_fo(&self) -> XTAL32K_WDT_CLK_FO_R {
        XTAL32K_WDT_CLK_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset the 32 kHz crystal watchdog by SW."]
    #[inline(always)]
    pub fn xtal32k_wdt_reset(&self) -> XTAL32K_WDT_RESET_R {
        XTAL32K_WDT_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to FPU the external clock of 32 kHz crystal."]
    #[inline(always)]
    pub fn xtal32k_ext_clk_fo(&self) -> XTAL32K_EXT_CLK_FO_R {
        XTAL32K_EXT_CLK_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_auto_backup(&self) -> XTAL32K_AUTO_BACKUP_R {
        XTAL32K_AUTO_BACKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_auto_restart(&self) -> XTAL32K_AUTO_RESTART_R {
        XTAL32K_AUTO_RESTART_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
    #[inline(always)]
    pub fn xtal32k_auto_return(&self) -> XTAL32K_AUTO_RETURN_R {
        XTAL32K_AUTO_RETURN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
    #[inline(always)]
    pub fn xtal32k_xpd_force(&self) -> XTAL32K_XPD_FORCE_R {
        XTAL32K_XPD_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Applies an internal clock to help the 32 kHz crystal to start."]
    #[inline(always)]
    pub fn enckinit_xtal_32k(&self) -> ENCKINIT_XTAL_32K_R {
        ENCKINIT_XTAL_32K_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: single-end buffer 1: differential buffer"]
    #[inline(always)]
    pub fn dbuf_xtal_32k(&self) -> DBUF_XTAL_32K_R {
        DBUF_XTAL_32K_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - xtal_32k gm control"]
    #[inline(always)]
    pub fn dgm_xtal_32k(&self) -> DGM_XTAL_32K_R {
        DGM_XTAL_32K_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - DRES_XTAL_32K"]
    #[inline(always)]
    pub fn dres_xtal_32k(&self) -> DRES_XTAL_32K_R {
        DRES_XTAL_32K_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - XPD_XTAL_32K"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&self) -> XPD_XTAL_32K_R {
        XPD_XTAL_32K_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DAC_XTAL_32K"]
    #[inline(always)]
    pub fn dac_xtal_32k(&self) -> DAC_XTAL_32K_R {
        DAC_XTAL_32K_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Stores the status of the 32 kHz watchdog."]
    #[inline(always)]
    pub fn wdt_state(&self) -> WDT_STATE_R {
        WDT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
    #[inline(always)]
    pub fn xtal32k_gpio_sel(&self) -> XTAL32K_GPIO_SEL_R {
        XTAL32K_GPIO_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the GPIO to power down the crystal oscillator."]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_XTL_CONF")
            .field(
                "xtal32k_wdt_en",
                &format_args!("{}", self.xtal32k_wdt_en().bit()),
            )
            .field(
                "xtal32k_wdt_clk_fo",
                &format_args!("{}", self.xtal32k_wdt_clk_fo().bit()),
            )
            .field(
                "xtal32k_wdt_reset",
                &format_args!("{}", self.xtal32k_wdt_reset().bit()),
            )
            .field(
                "xtal32k_ext_clk_fo",
                &format_args!("{}", self.xtal32k_ext_clk_fo().bit()),
            )
            .field(
                "xtal32k_auto_backup",
                &format_args!("{}", self.xtal32k_auto_backup().bit()),
            )
            .field(
                "xtal32k_auto_restart",
                &format_args!("{}", self.xtal32k_auto_restart().bit()),
            )
            .field(
                "xtal32k_auto_return",
                &format_args!("{}", self.xtal32k_auto_return().bit()),
            )
            .field(
                "xtal32k_xpd_force",
                &format_args!("{}", self.xtal32k_xpd_force().bit()),
            )
            .field(
                "enckinit_xtal_32k",
                &format_args!("{}", self.enckinit_xtal_32k().bit()),
            )
            .field(
                "dbuf_xtal_32k",
                &format_args!("{}", self.dbuf_xtal_32k().bit()),
            )
            .field(
                "dgm_xtal_32k",
                &format_args!("{}", self.dgm_xtal_32k().bits()),
            )
            .field(
                "dres_xtal_32k",
                &format_args!("{}", self.dres_xtal_32k().bits()),
            )
            .field(
                "xpd_xtal_32k",
                &format_args!("{}", self.xpd_xtal_32k().bit()),
            )
            .field(
                "dac_xtal_32k",
                &format_args!("{}", self.dac_xtal_32k().bits()),
            )
            .field("wdt_state", &format_args!("{}", self.wdt_state().bits()))
            .field(
                "xtal32k_gpio_sel",
                &format_args!("{}", self.xtal32k_gpio_sel().bit()),
            )
            .field(
                "xtl_ext_ctr_lv",
                &format_args!("{}", self.xtl_ext_ctr_lv().bit()),
            )
            .field(
                "xtl_ext_ctr_en",
                &format_args!("{}", self.xtl_ext_ctr_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_XTL_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable the 32 kHz crystal watchdog."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_wdt_en(&mut self) -> XTAL32K_WDT_EN_W<EXT_XTL_CONF_SPEC, 0> {
        XTAL32K_WDT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to FPU the 32 kHz crystal watchdog clock."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_wdt_clk_fo(&mut self) -> XTAL32K_WDT_CLK_FO_W<EXT_XTL_CONF_SPEC, 1> {
        XTAL32K_WDT_CLK_FO_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset the 32 kHz crystal watchdog by SW."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_wdt_reset(&mut self) -> XTAL32K_WDT_RESET_W<EXT_XTL_CONF_SPEC, 2> {
        XTAL32K_WDT_RESET_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to FPU the external clock of 32 kHz crystal."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_ext_clk_fo(&mut self) -> XTAL32K_EXT_CLK_FO_W<EXT_XTL_CONF_SPEC, 3> {
        XTAL32K_EXT_CLK_FO_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_auto_backup(&mut self) -> XTAL32K_AUTO_BACKUP_W<EXT_XTL_CONF_SPEC, 4> {
        XTAL32K_AUTO_BACKUP_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_auto_restart(&mut self) -> XTAL32K_AUTO_RESTART_W<EXT_XTL_CONF_SPEC, 5> {
        XTAL32K_AUTO_RESTART_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_auto_return(&mut self) -> XTAL32K_AUTO_RETURN_W<EXT_XTL_CONF_SPEC, 6> {
        XTAL32K_AUTO_RETURN_W::new(self)
    }
    #[doc = "Bit 7 - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_xpd_force(&mut self) -> XTAL32K_XPD_FORCE_W<EXT_XTL_CONF_SPEC, 7> {
        XTAL32K_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 8 - Applies an internal clock to help the 32 kHz crystal to start."]
    #[inline(always)]
    #[must_use]
    pub fn enckinit_xtal_32k(&mut self) -> ENCKINIT_XTAL_32K_W<EXT_XTL_CONF_SPEC, 8> {
        ENCKINIT_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 9 - 0: single-end buffer 1: differential buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dbuf_xtal_32k(&mut self) -> DBUF_XTAL_32K_W<EXT_XTL_CONF_SPEC, 9> {
        DBUF_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 10:12 - xtal_32k gm control"]
    #[inline(always)]
    #[must_use]
    pub fn dgm_xtal_32k(&mut self) -> DGM_XTAL_32K_W<EXT_XTL_CONF_SPEC, 10> {
        DGM_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 13:15 - DRES_XTAL_32K"]
    #[inline(always)]
    #[must_use]
    pub fn dres_xtal_32k(&mut self) -> DRES_XTAL_32K_W<EXT_XTL_CONF_SPEC, 13> {
        DRES_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 16 - XPD_XTAL_32K"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_xtal_32k(&mut self) -> XPD_XTAL_32K_W<EXT_XTL_CONF_SPEC, 16> {
        XPD_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 17:19 - DAC_XTAL_32K"]
    #[inline(always)]
    #[must_use]
    pub fn dac_xtal_32k(&mut self) -> DAC_XTAL_32K_W<EXT_XTL_CONF_SPEC, 17> {
        DAC_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 23 - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_gpio_sel(&mut self) -> XTAL32K_GPIO_SEL_W<EXT_XTL_CONF_SPEC, 23> {
        XTAL32K_GPIO_SEL_W::new(self)
    }
    #[doc = "Bit 30 - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
    #[inline(always)]
    #[must_use]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W<EXT_XTL_CONF_SPEC, 30> {
        XTL_EXT_CTR_LV_W::new(self)
    }
    #[doc = "Bit 31 - Enables the GPIO to power down the crystal oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W<EXT_XTL_CONF_SPEC, 31> {
        XTL_EXT_CTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "32 kHz crystal oscillator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_XTL_CONF_SPEC;
impl crate::RegisterSpec for EXT_XTL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_xtl_conf::R`](R) reader structure"]
impl crate::Readable for EXT_XTL_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_xtl_conf::W`](W) writer structure"]
impl crate::Writable for EXT_XTL_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_XTL_CONF to value 0x0006_6c80"]
impl crate::Resettable for EXT_XTL_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_6c80;
}
