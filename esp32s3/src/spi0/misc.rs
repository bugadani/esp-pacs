#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `FSUB_PIN` reader - Flash is connected to SPI SUBPIN bus."]
pub type FSUB_PIN_R = crate::BitReader;
#[doc = "Field `FSUB_PIN` writer - Flash is connected to SPI SUBPIN bus."]
pub type FSUB_PIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSUB_PIN` reader - Ext_RAM is connected to SPI SUBPIN bus."]
pub type SSUB_PIN_R = crate::BitReader;
#[doc = "Field `SSUB_PIN` writer - Ext_RAM is connected to SPI SUBPIN bus."]
pub type SSUB_PIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 7 - Flash is connected to SPI SUBPIN bus."]
    #[inline(always)]
    pub fn fsub_pin(&self) -> FSUB_PIN_R {
        FSUB_PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ext_RAM is connected to SPI SUBPIN bus."]
    #[inline(always)]
    pub fn ssub_pin(&self) -> SSUB_PIN_R {
        SSUB_PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("fsub_pin", &format_args!("{}", self.fsub_pin().bit()))
            .field("ssub_pin", &format_args!("{}", self.ssub_pin().bit()))
            .field(
                "ck_idle_edge",
                &format_args!("{}", self.ck_idle_edge().bit()),
            )
            .field(
                "cs_keep_active",
                &format_args!("{}", self.cs_keep_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 7 - Flash is connected to SPI SUBPIN bus."]
    #[inline(always)]
    #[must_use]
    pub fn fsub_pin(&mut self) -> FSUB_PIN_W<MISC_SPEC, 7> {
        FSUB_PIN_W::new(self)
    }
    #[doc = "Bit 8 - Ext_RAM is connected to SPI SUBPIN bus."]
    #[inline(always)]
    #[must_use]
    pub fn ssub_pin(&mut self) -> SSUB_PIN_W<MISC_SPEC, 8> {
        SSUB_PIN_W::new(self)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC, 9> {
        CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC, 10> {
        CS_KEEP_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
