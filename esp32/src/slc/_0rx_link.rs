#[doc = "Register `_0RX_LINK` reader"]
pub type R = crate::R<_0RX_LINK_SPEC>;
#[doc = "Register `_0RX_LINK` writer"]
pub type W = crate::W<_0RX_LINK_SPEC>;
#[doc = "Field `SLC0_RXLINK_ADDR` reader - "]
pub type SLC0_RXLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SLC0_RXLINK_ADDR` writer - "]
pub type SLC0_RXLINK_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `SLC0_RXLINK_STOP` reader - "]
pub type SLC0_RXLINK_STOP_R = crate::BitReader;
#[doc = "Field `SLC0_RXLINK_STOP` writer - "]
pub type SLC0_RXLINK_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RXLINK_START` reader - "]
pub type SLC0_RXLINK_START_R = crate::BitReader;
#[doc = "Field `SLC0_RXLINK_START` writer - "]
pub type SLC0_RXLINK_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RXLINK_RESTART` reader - "]
pub type SLC0_RXLINK_RESTART_R = crate::BitReader;
#[doc = "Field `SLC0_RXLINK_RESTART` writer - "]
pub type SLC0_RXLINK_RESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RXLINK_PARK` reader - "]
pub type SLC0_RXLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_rxlink_addr(&self) -> SLC0_RXLINK_ADDR_R {
        SLC0_RXLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc0_rxlink_stop(&self) -> SLC0_RXLINK_STOP_R {
        SLC0_RXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc0_rxlink_start(&self) -> SLC0_RXLINK_START_R {
        SLC0_RXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc0_rxlink_restart(&self) -> SLC0_RXLINK_RESTART_R {
        SLC0_RXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc0_rxlink_park(&self) -> SLC0_RXLINK_PARK_R {
        SLC0_RXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0RX_LINK")
            .field(
                "slc0_rxlink_addr",
                &format_args!("{}", self.slc0_rxlink_addr().bits()),
            )
            .field(
                "slc0_rxlink_stop",
                &format_args!("{}", self.slc0_rxlink_stop().bit()),
            )
            .field(
                "slc0_rxlink_start",
                &format_args!("{}", self.slc0_rxlink_start().bit()),
            )
            .field(
                "slc0_rxlink_restart",
                &format_args!("{}", self.slc0_rxlink_restart().bit()),
            )
            .field(
                "slc0_rxlink_park",
                &format_args!("{}", self.slc0_rxlink_park().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0RX_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxlink_addr(&mut self) -> SLC0_RXLINK_ADDR_W<_0RX_LINK_SPEC, 0> {
        SLC0_RXLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxlink_stop(&mut self) -> SLC0_RXLINK_STOP_W<_0RX_LINK_SPEC, 28> {
        SLC0_RXLINK_STOP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxlink_start(&mut self) -> SLC0_RXLINK_START_W<_0RX_LINK_SPEC, 29> {
        SLC0_RXLINK_START_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxlink_restart(&mut self) -> SLC0_RXLINK_RESTART_W<_0RX_LINK_SPEC, 30> {
        SLC0_RXLINK_RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0rx_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0rx_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0RX_LINK_SPEC;
impl crate::RegisterSpec for _0RX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0rx_link::R`](R) reader structure"]
impl crate::Readable for _0RX_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0rx_link::W`](W) writer structure"]
impl crate::Writable for _0RX_LINK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0RX_LINK to value 0"]
impl crate::Resettable for _0RX_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
