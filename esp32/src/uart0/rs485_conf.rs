#[doc = "Register `RS485_CONF` reader"]
pub type R = crate::R<RS485_CONF_SPEC>;
#[doc = "Register `RS485_CONF` writer"]
pub type W = crate::W<RS485_CONF_SPEC>;
#[doc = "Field `RS485_EN` reader - Set this bit to choose rs485 mode."]
pub type RS485_EN_R = crate::BitReader;
#[doc = "Field `RS485_EN` writer - Set this bit to choose rs485 mode."]
pub type RS485_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_R = crate::BitReader;
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_R = crate::BitReader;
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RS485TX_RX_EN` reader - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
pub type RS485TX_RX_EN_R = crate::BitReader;
#[doc = "Field `RS485TX_RX_EN` writer - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
pub type RS485TX_RX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RS485RXBY_TX_EN` reader - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
pub type RS485RXBY_TX_EN_R = crate::BitReader;
#[doc = "Field `RS485RXBY_TX_EN` writer - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
pub type RS485RXBY_TX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RS485_RX_DLY_NUM` reader - This register is used to delay the receiver's internal data signal."]
pub type RS485_RX_DLY_NUM_R = crate::BitReader;
#[doc = "Field `RS485_RX_DLY_NUM` writer - This register is used to delay the receiver's internal data signal."]
pub type RS485_RX_DLY_NUM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RS485_TX_DLY_NUM` reader - This register is used to delay the transmitter's internal data signal."]
pub type RS485_TX_DLY_NUM_R = crate::FieldReader;
#[doc = "Field `RS485_TX_DLY_NUM` writer - This register is used to delay the transmitter's internal data signal."]
pub type RS485_TX_DLY_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to choose rs485 mode."]
    #[inline(always)]
    pub fn rs485_en(&self) -> RS485_EN_R {
        RS485_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&self) -> RS485TX_RX_EN_R {
        RS485TX_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
    #[inline(always)]
    pub fn rs485rxby_tx_en(&self) -> RS485RXBY_TX_EN_R {
        RS485RXBY_TX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn rs485_rx_dly_num(&self) -> RS485_RX_DLY_NUM_R {
        RS485_RX_DLY_NUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn rs485_tx_dly_num(&self) -> RS485_TX_DLY_NUM_R {
        RS485_TX_DLY_NUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485_CONF")
            .field("rs485_en", &format_args!("{}", self.rs485_en().bit()))
            .field("dl0_en", &format_args!("{}", self.dl0_en().bit()))
            .field("dl1_en", &format_args!("{}", self.dl1_en().bit()))
            .field(
                "rs485tx_rx_en",
                &format_args!("{}", self.rs485tx_rx_en().bit()),
            )
            .field(
                "rs485rxby_tx_en",
                &format_args!("{}", self.rs485rxby_tx_en().bit()),
            )
            .field(
                "rs485_rx_dly_num",
                &format_args!("{}", self.rs485_rx_dly_num().bit()),
            )
            .field(
                "rs485_tx_dly_num",
                &format_args!("{}", self.rs485_tx_dly_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RS485_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to choose rs485 mode."]
    #[inline(always)]
    #[must_use]
    pub fn rs485_en(&mut self) -> RS485_EN_W<RS485_CONF_SPEC, 0> {
        RS485_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl0_en(&mut self) -> DL0_EN_W<RS485_CONF_SPEC, 1> {
        DL0_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl1_en(&mut self) -> DL1_EN_W<RS485_CONF_SPEC, 2> {
        DL1_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
    #[inline(always)]
    #[must_use]
    pub fn rs485tx_rx_en(&mut self) -> RS485TX_RX_EN_W<RS485_CONF_SPEC, 3> {
        RS485TX_RX_EN_W::new(self)
    }
    #[doc = "Bit 4 - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
    #[inline(always)]
    #[must_use]
    pub fn rs485rxby_tx_en(&mut self) -> RS485RXBY_TX_EN_W<RS485_CONF_SPEC, 4> {
        RS485RXBY_TX_EN_W::new(self)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    #[must_use]
    pub fn rs485_rx_dly_num(&mut self) -> RS485_RX_DLY_NUM_W<RS485_CONF_SPEC, 5> {
        RS485_RX_DLY_NUM_W::new(self)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    #[must_use]
    pub fn rs485_tx_dly_num(&mut self) -> RS485_TX_DLY_NUM_W<RS485_CONF_SPEC, 6> {
        RS485_TX_DLY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RS485_CONF_SPEC;
impl crate::RegisterSpec for RS485_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485_conf::R`](R) reader structure"]
impl crate::Readable for RS485_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rs485_conf::W`](W) writer structure"]
impl crate::Writable for RS485_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485_CONF to value 0"]
impl crate::Resettable for RS485_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
