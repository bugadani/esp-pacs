#[doc = "Register `CFG_DATA7` reader"]
pub type R = crate::R<CFG_DATA7_SPEC>;
#[doc = "Register `CFG_DATA7` writer"]
pub type W = crate::W<CFG_DATA7_SPEC>;
#[doc = "Field `PIN_STATE` reader - "]
pub type PIN_STATE_R = crate::FieldReader;
#[doc = "Field `PIN_STATE` writer - "]
pub type PIN_STATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CHIP_STATE` reader - "]
pub type CHIP_STATE_R = crate::FieldReader;
#[doc = "Field `CHIP_STATE` writer - "]
pub type CHIP_STATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SDIO_RST` reader - "]
pub type SDIO_RST_R = crate::BitReader;
#[doc = "Field `SDIO_RST` writer - "]
pub type SDIO_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO_IOREADY0` reader - "]
pub type SDIO_IOREADY0_R = crate::BitReader;
#[doc = "Field `SDIO_IOREADY0` writer - "]
pub type SDIO_IOREADY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pin_state(&self) -> PIN_STATE_R {
        PIN_STATE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chip_state(&self) -> CHIP_STATE_R {
        CHIP_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sdio_ioready0(&self) -> SDIO_IOREADY0_R {
        SDIO_IOREADY0_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA7")
            .field("pin_state", &format_args!("{}", self.pin_state().bits()))
            .field("chip_state", &format_args!("{}", self.chip_state().bits()))
            .field("sdio_rst", &format_args!("{}", self.sdio_rst().bit()))
            .field(
                "sdio_ioready0",
                &format_args!("{}", self.sdio_ioready0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_DATA7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pin_state(&mut self) -> PIN_STATE_W<CFG_DATA7_SPEC, 0> {
        PIN_STATE_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn chip_state(&mut self) -> CHIP_STATE_W<CFG_DATA7_SPEC, 8> {
        CHIP_STATE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W<CFG_DATA7_SPEC, 16> {
        SDIO_RST_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready0(&mut self) -> SDIO_IOREADY0_W<CFG_DATA7_SPEC, 17> {
        SDIO_IOREADY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_DATA7_SPEC;
impl crate::RegisterSpec for CFG_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data7::R`](R) reader structure"]
impl crate::Readable for CFG_DATA7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_data7::W`](W) writer structure"]
impl crate::Writable for CFG_DATA7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DATA7 to value 0x0002_0000"]
impl crate::Resettable for CFG_DATA7_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0000;
}
