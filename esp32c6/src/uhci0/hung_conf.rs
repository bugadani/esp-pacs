#[doc = "Register `HUNG_CONF` reader"]
pub type R = crate::R<HUNG_CONF_SPEC>;
#[doc = "Register `HUNG_CONF` writer"]
pub type W = crate::W<HUNG_CONF_SPEC>;
#[doc = "Field `TXFIFO_TIMEOUT` reader - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
pub type TXFIFO_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT` writer - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
pub type TXFIFO_TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` reader - Configures the maximum counter value."]
pub type TXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` writer - Configures the maximum counter value."]
pub type TXFIFO_TIMEOUT_SHIFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` reader - Set this bit to enable TX FIFO timeout when receiving."]
pub type TXFIFO_TIMEOUT_ENA_R = crate::BitReader;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` writer - Set this bit to enable TX FIFO timeout when receiving."]
pub type TXFIFO_TIMEOUT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFIFO_TIMEOUT` reader - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
pub type RXFIFO_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT` writer - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
pub type RXFIFO_TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` reader - Configures the maximum counter value."]
pub type RXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` writer - Configures the maximum counter value."]
pub type RXFIFO_TIMEOUT_SHIFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` reader - Set this bit to enable TX FIFO timeout when DMA sending data."]
pub type RXFIFO_TIMEOUT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` writer - Set this bit to enable TX FIFO timeout when DMA sending data."]
pub type RXFIFO_TIMEOUT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
    #[inline(always)]
    pub fn txfifo_timeout(&self) -> TXFIFO_TIMEOUT_R {
        TXFIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Configures the maximum counter value."]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&self) -> TXFIFO_TIMEOUT_SHIFT_R {
        TXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Set this bit to enable TX FIFO timeout when receiving."]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&self) -> TXFIFO_TIMEOUT_ENA_R {
        TXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
    #[inline(always)]
    pub fn rxfifo_timeout(&self) -> RXFIFO_TIMEOUT_R {
        RXFIFO_TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:22 - Configures the maximum counter value."]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&self) -> RXFIFO_TIMEOUT_SHIFT_R {
        RXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Set this bit to enable TX FIFO timeout when DMA sending data."]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&self) -> RXFIFO_TIMEOUT_ENA_R {
        RXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUNG_CONF")
            .field(
                "txfifo_timeout",
                &format_args!("{}", self.txfifo_timeout().bits()),
            )
            .field(
                "txfifo_timeout_shift",
                &format_args!("{}", self.txfifo_timeout_shift().bits()),
            )
            .field(
                "txfifo_timeout_ena",
                &format_args!("{}", self.txfifo_timeout_ena().bit()),
            )
            .field(
                "rxfifo_timeout",
                &format_args!("{}", self.rxfifo_timeout().bits()),
            )
            .field(
                "rxfifo_timeout_shift",
                &format_args!("{}", self.rxfifo_timeout_shift().bits()),
            )
            .field(
                "rxfifo_timeout_ena",
                &format_args!("{}", self.rxfifo_timeout_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HUNG_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout(&mut self) -> TXFIFO_TIMEOUT_W<HUNG_CONF_SPEC, 0> {
        TXFIFO_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 8:10 - Configures the maximum counter value."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout_shift(&mut self) -> TXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC, 8> {
        TXFIFO_TIMEOUT_SHIFT_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to enable TX FIFO timeout when receiving."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout_ena(&mut self) -> TXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC, 11> {
        TXFIFO_TIMEOUT_ENA_W::new(self)
    }
    #[doc = "Bits 12:19 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout(&mut self) -> RXFIFO_TIMEOUT_W<HUNG_CONF_SPEC, 12> {
        RXFIFO_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 20:22 - Configures the maximum counter value."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout_shift(&mut self) -> RXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC, 20> {
        RXFIFO_TIMEOUT_SHIFT_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to enable TX FIFO timeout when DMA sending data."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout_ena(&mut self) -> RXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC, 23> {
        RXFIFO_TIMEOUT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UHCI Hung Configuration Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hung_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hung_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUNG_CONF_SPEC;
impl crate::RegisterSpec for HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hung_conf::R`](R) reader structure"]
impl crate::Readable for HUNG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hung_conf::W`](W) writer structure"]
impl crate::Writable for HUNG_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HUNG_CONF to value 0x0081_0810"]
impl crate::Resettable for HUNG_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0081_0810;
}
