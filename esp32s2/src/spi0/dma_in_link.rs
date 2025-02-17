#[doc = "Register `DMA_IN_LINK` reader"]
pub type R = crate::R<DMA_IN_LINK_SPEC>;
#[doc = "Register `DMA_IN_LINK` writer"]
pub type W = crate::W<DMA_IN_LINK_SPEC>;
#[doc = "Field `INLINK_ADDR` reader - The address of the first inlink descriptor."]
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - The address of the first inlink descriptor."]
pub type INLINK_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `INLINK_AUTO_RET` reader - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
pub type INLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET` writer - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
pub type INLINK_AUTO_RET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INLINK_STOP` reader - Set the bit to stop to use inlink descriptor."]
pub type INLINK_STOP_R = crate::BitReader;
#[doc = "Field `INLINK_STOP` writer - Set the bit to stop to use inlink descriptor."]
pub type INLINK_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INLINK_START` reader - Set the bit to start to use inlink descriptor."]
pub type INLINK_START_R = crate::BitReader;
#[doc = "Field `INLINK_START` writer - Set the bit to start to use inlink descriptor."]
pub type INLINK_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INLINK_RESTART` reader - Set the bit to mount on new inlink descriptors."]
pub type INLINK_RESTART_R = crate::BitReader;
#[doc = "Field `INLINK_RESTART` writer - Set the bit to mount on new inlink descriptors."]
pub type INLINK_RESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_RX_ENA` reader - SPI DMA read data status bit."]
pub type DMA_RX_ENA_R = crate::BitReader;
#[doc = "Field `DMA_RX_ENA` writer - SPI DMA read data status bit."]
pub type DMA_RX_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:19 - The address of the first inlink descriptor."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
    #[inline(always)]
    pub fn inlink_auto_ret(&self) -> INLINK_AUTO_RET_R {
        INLINK_AUTO_RET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28 - Set the bit to stop to use inlink descriptor."]
    #[inline(always)]
    pub fn inlink_stop(&self) -> INLINK_STOP_R {
        INLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set the bit to start to use inlink descriptor."]
    #[inline(always)]
    pub fn inlink_start(&self) -> INLINK_START_R {
        INLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set the bit to mount on new inlink descriptors."]
    #[inline(always)]
    pub fn inlink_restart(&self) -> INLINK_RESTART_R {
        INLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI DMA read data status bit."]
    #[inline(always)]
    pub fn dma_rx_ena(&self) -> DMA_RX_ENA_R {
        DMA_RX_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_LINK")
            .field(
                "inlink_addr",
                &format_args!("{}", self.inlink_addr().bits()),
            )
            .field(
                "inlink_auto_ret",
                &format_args!("{}", self.inlink_auto_ret().bit()),
            )
            .field("inlink_stop", &format_args!("{}", self.inlink_stop().bit()))
            .field(
                "inlink_start",
                &format_args!("{}", self.inlink_start().bit()),
            )
            .field(
                "inlink_restart",
                &format_args!("{}", self.inlink_restart().bit()),
            )
            .field("dma_rx_ena", &format_args!("{}", self.dma_rx_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_IN_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - The address of the first inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<DMA_IN_LINK_SPEC, 0> {
        INLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 20 - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_auto_ret(&mut self) -> INLINK_AUTO_RET_W<DMA_IN_LINK_SPEC, 20> {
        INLINK_AUTO_RET_W::new(self)
    }
    #[doc = "Bit 28 - Set the bit to stop to use inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W<DMA_IN_LINK_SPEC, 28> {
        INLINK_STOP_W::new(self)
    }
    #[doc = "Bit 29 - Set the bit to start to use inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_start(&mut self) -> INLINK_START_W<DMA_IN_LINK_SPEC, 29> {
        INLINK_START_W::new(self)
    }
    #[doc = "Bit 30 - Set the bit to mount on new inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W<DMA_IN_LINK_SPEC, 30> {
        INLINK_RESTART_W::new(self)
    }
    #[doc = "Bit 31 - SPI DMA read data status bit."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_ena(&mut self) -> DMA_RX_ENA_W<DMA_IN_LINK_SPEC, 31> {
        DMA_RX_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI DMA RX link configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IN_LINK_SPEC;
impl crate::RegisterSpec for DMA_IN_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_in_link::R`](R) reader structure"]
impl crate::Readable for DMA_IN_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_in_link::W`](W) writer structure"]
impl crate::Writable for DMA_IN_LINK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_IN_LINK to value 0"]
impl crate::Resettable for DMA_IN_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
