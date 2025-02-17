#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `CLK_ETM_EN` reader - "]
pub type CLK_ETM_EN_R = crate::BitReader;
#[doc = "Field `CLK_ETM_EN` writer - "]
pub type CLK_ETM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_ZB_APB_EN` reader - "]
pub type CLK_ZB_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_ZB_APB_EN` writer - "]
pub type CLK_ZB_APB_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_ZB_MAC_EN` reader - "]
pub type CLK_ZB_MAC_EN_R = crate::BitReader;
#[doc = "Field `CLK_ZB_MAC_EN` writer - "]
pub type CLK_ZB_MAC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_MODEM_SEC_ECB_EN` reader - "]
pub type CLK_MODEM_SEC_ECB_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_ECB_EN` writer - "]
pub type CLK_MODEM_SEC_ECB_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_MODEM_SEC_CCM_EN` reader - "]
pub type CLK_MODEM_SEC_CCM_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_CCM_EN` writer - "]
pub type CLK_MODEM_SEC_CCM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_MODEM_SEC_BAH_EN` reader - "]
pub type CLK_MODEM_SEC_BAH_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_BAH_EN` writer - "]
pub type CLK_MODEM_SEC_BAH_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_MODEM_SEC_APB_EN` reader - "]
pub type CLK_MODEM_SEC_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_APB_EN` writer - "]
pub type CLK_MODEM_SEC_APB_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_MODEM_SEC_EN` reader - "]
pub type CLK_MODEM_SEC_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_EN` writer - "]
pub type CLK_MODEM_SEC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_BLE_TIMER_APB_EN` reader - "]
pub type CLK_BLE_TIMER_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_BLE_TIMER_APB_EN` writer - "]
pub type CLK_BLE_TIMER_APB_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_BLE_TIMER_EN` reader - "]
pub type CLK_BLE_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CLK_BLE_TIMER_EN` writer - "]
pub type CLK_BLE_TIMER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_DATA_DUMP_EN` reader - "]
pub type CLK_DATA_DUMP_EN_R = crate::BitReader;
#[doc = "Field `CLK_DATA_DUMP_EN` writer - "]
pub type CLK_DATA_DUMP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_etm_en(&self) -> CLK_ETM_EN_R {
        CLK_ETM_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_zb_apb_en(&self) -> CLK_ZB_APB_EN_R {
        CLK_ZB_APB_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_zb_mac_en(&self) -> CLK_ZB_MAC_EN_R {
        CLK_ZB_MAC_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_modem_sec_ecb_en(&self) -> CLK_MODEM_SEC_ECB_EN_R {
        CLK_MODEM_SEC_ECB_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_modem_sec_ccm_en(&self) -> CLK_MODEM_SEC_CCM_EN_R {
        CLK_MODEM_SEC_CCM_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_modem_sec_bah_en(&self) -> CLK_MODEM_SEC_BAH_EN_R {
        CLK_MODEM_SEC_BAH_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_modem_sec_apb_en(&self) -> CLK_MODEM_SEC_APB_EN_R {
        CLK_MODEM_SEC_APB_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_modem_sec_en(&self) -> CLK_MODEM_SEC_EN_R {
        CLK_MODEM_SEC_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_ble_timer_apb_en(&self) -> CLK_BLE_TIMER_APB_EN_R {
        CLK_BLE_TIMER_APB_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_ble_timer_en(&self) -> CLK_BLE_TIMER_EN_R {
        CLK_BLE_TIMER_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_data_dump_en(&self) -> CLK_DATA_DUMP_EN_R {
        CLK_DATA_DUMP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("clk_etm_en", &format_args!("{}", self.clk_etm_en().bit()))
            .field(
                "clk_zb_apb_en",
                &format_args!("{}", self.clk_zb_apb_en().bit()),
            )
            .field(
                "clk_zb_mac_en",
                &format_args!("{}", self.clk_zb_mac_en().bit()),
            )
            .field(
                "clk_modem_sec_ecb_en",
                &format_args!("{}", self.clk_modem_sec_ecb_en().bit()),
            )
            .field(
                "clk_modem_sec_ccm_en",
                &format_args!("{}", self.clk_modem_sec_ccm_en().bit()),
            )
            .field(
                "clk_modem_sec_bah_en",
                &format_args!("{}", self.clk_modem_sec_bah_en().bit()),
            )
            .field(
                "clk_modem_sec_apb_en",
                &format_args!("{}", self.clk_modem_sec_apb_en().bit()),
            )
            .field(
                "clk_modem_sec_en",
                &format_args!("{}", self.clk_modem_sec_en().bit()),
            )
            .field(
                "clk_ble_timer_apb_en",
                &format_args!("{}", self.clk_ble_timer_apb_en().bit()),
            )
            .field(
                "clk_ble_timer_en",
                &format_args!("{}", self.clk_ble_timer_en().bit()),
            )
            .field(
                "clk_data_dump_en",
                &format_args!("{}", self.clk_data_dump_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn clk_etm_en(&mut self) -> CLK_ETM_EN_W<CLK_CONF_SPEC, 21> {
        CLK_ETM_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn clk_zb_apb_en(&mut self) -> CLK_ZB_APB_EN_W<CLK_CONF_SPEC, 22> {
        CLK_ZB_APB_EN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn clk_zb_mac_en(&mut self) -> CLK_ZB_MAC_EN_W<CLK_CONF_SPEC, 23> {
        CLK_ZB_MAC_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_ecb_en(&mut self) -> CLK_MODEM_SEC_ECB_EN_W<CLK_CONF_SPEC, 24> {
        CLK_MODEM_SEC_ECB_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_ccm_en(&mut self) -> CLK_MODEM_SEC_CCM_EN_W<CLK_CONF_SPEC, 25> {
        CLK_MODEM_SEC_CCM_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_bah_en(&mut self) -> CLK_MODEM_SEC_BAH_EN_W<CLK_CONF_SPEC, 26> {
        CLK_MODEM_SEC_BAH_EN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_apb_en(&mut self) -> CLK_MODEM_SEC_APB_EN_W<CLK_CONF_SPEC, 27> {
        CLK_MODEM_SEC_APB_EN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_en(&mut self) -> CLK_MODEM_SEC_EN_W<CLK_CONF_SPEC, 28> {
        CLK_MODEM_SEC_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ble_timer_apb_en(&mut self) -> CLK_BLE_TIMER_APB_EN_W<CLK_CONF_SPEC, 29> {
        CLK_BLE_TIMER_APB_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ble_timer_en(&mut self) -> CLK_BLE_TIMER_EN_W<CLK_CONF_SPEC, 30> {
        CLK_BLE_TIMER_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn clk_data_dump_en(&mut self) -> CLK_DATA_DUMP_EN_W<CLK_CONF_SPEC, 31> {
        CLK_DATA_DUMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
