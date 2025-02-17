#[doc = "Register `IDLE_CONF_SYNC` reader"]
pub type R = crate::R<IDLE_CONF_SYNC_SPEC>;
#[doc = "Register `IDLE_CONF_SYNC` writer"]
pub type W = crate::W<IDLE_CONF_SYNC_SPEC>;
#[doc = "Field `RX_IDLE_THRHD` reader - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
pub type RX_IDLE_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `RX_IDLE_THRHD` writer - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
pub type RX_IDLE_THRHD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `TX_IDLE_NUM` reader - This register is used to configure the duration time between transfers."]
pub type TX_IDLE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_IDLE_NUM` writer - This register is used to configure the duration time between transfers."]
pub type TX_IDLE_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> RX_IDLE_THRHD_R {
        RX_IDLE_THRHD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    pub fn tx_idle_num(&self) -> TX_IDLE_NUM_R {
        TX_IDLE_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE_CONF_SYNC")
            .field(
                "rx_idle_thrhd",
                &format_args!("{}", self.rx_idle_thrhd().bits()),
            )
            .field(
                "tx_idle_num",
                &format_args!("{}", self.tx_idle_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDLE_CONF_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
    #[inline(always)]
    #[must_use]
    pub fn rx_idle_thrhd(&mut self) -> RX_IDLE_THRHD_W<IDLE_CONF_SYNC_SPEC, 0> {
        RX_IDLE_THRHD_W::new(self)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle_num(&mut self) -> TX_IDLE_NUM_W<IDLE_CONF_SYNC_SPEC, 10> {
        TX_IDLE_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Frame-end idle configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_conf_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_conf_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDLE_CONF_SYNC_SPEC;
impl crate::RegisterSpec for IDLE_CONF_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle_conf_sync::R`](R) reader structure"]
impl crate::Readable for IDLE_CONF_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idle_conf_sync::W`](W) writer structure"]
impl crate::Writable for IDLE_CONF_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDLE_CONF_SYNC to value 0x0004_0100"]
impl crate::Resettable for IDLE_CONF_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0100;
}
