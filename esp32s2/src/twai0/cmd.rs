#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `TX_REQ` writer - Set the bit to 1 to allow the driving nodes start transmission."]
pub type TX_REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABORT_TX` writer - Set the bit to 1 to cancel a pending transmission request."]
pub type ABORT_TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RELEASE_BUF` writer - Set the bit to 1 to release the RX buffer."]
pub type RELEASE_BUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_OVERRUN` writer - Set the bit to 1 to clear the data overrun status bit."]
pub type CLR_OVERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SELF_RX_REQ` writer - Self reception request command. Set the bit to 1 to allow a message be transmitted and received simultaneously."]
pub type SELF_RX_REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to 1 to allow the driving nodes start transmission."]
    #[inline(always)]
    #[must_use]
    pub fn tx_req(&mut self) -> TX_REQ_W<CMD_SPEC, 0> {
        TX_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Set the bit to 1 to cancel a pending transmission request."]
    #[inline(always)]
    #[must_use]
    pub fn abort_tx(&mut self) -> ABORT_TX_W<CMD_SPEC, 1> {
        ABORT_TX_W::new(self)
    }
    #[doc = "Bit 2 - Set the bit to 1 to release the RX buffer."]
    #[inline(always)]
    #[must_use]
    pub fn release_buf(&mut self) -> RELEASE_BUF_W<CMD_SPEC, 2> {
        RELEASE_BUF_W::new(self)
    }
    #[doc = "Bit 3 - Set the bit to 1 to clear the data overrun status bit."]
    #[inline(always)]
    #[must_use]
    pub fn clr_overrun(&mut self) -> CLR_OVERRUN_W<CMD_SPEC, 3> {
        CLR_OVERRUN_W::new(self)
    }
    #[doc = "Bit 4 - Self reception request command. Set the bit to 1 to allow a message be transmitted and received simultaneously."]
    #[inline(always)]
    #[must_use]
    pub fn self_rx_req(&mut self) -> SELF_RX_REQ_W<CMD_SPEC, 4> {
        SELF_RX_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
