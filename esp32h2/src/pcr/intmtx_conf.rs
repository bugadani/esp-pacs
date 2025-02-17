#[doc = "Register `INTMTX_CONF` reader"]
pub type R = crate::R<INTMTX_CONF_SPEC>;
#[doc = "Register `INTMTX_CONF` writer"]
pub type W = crate::W<INTMTX_CONF_SPEC>;
#[doc = "Field `INTMTX_CLK_EN` reader - Set 1 to enable intmtx clock"]
pub type INTMTX_CLK_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_CLK_EN` writer - Set 1 to enable intmtx clock"]
pub type INTMTX_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTMTX_RST_EN` reader - Set 0 to reset intmtx module"]
pub type INTMTX_RST_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_RST_EN` writer - Set 0 to reset intmtx module"]
pub type INTMTX_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTMTX_READY` reader - Query this field after reset intmtx module"]
pub type INTMTX_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable intmtx clock"]
    #[inline(always)]
    pub fn intmtx_clk_en(&self) -> INTMTX_CLK_EN_R {
        INTMTX_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset intmtx module"]
    #[inline(always)]
    pub fn intmtx_rst_en(&self) -> INTMTX_RST_EN_R {
        INTMTX_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset intmtx module"]
    #[inline(always)]
    pub fn intmtx_ready(&self) -> INTMTX_READY_R {
        INTMTX_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMTX_CONF")
            .field(
                "intmtx_clk_en",
                &format_args!("{}", self.intmtx_clk_en().bit()),
            )
            .field(
                "intmtx_rst_en",
                &format_args!("{}", self.intmtx_rst_en().bit()),
            )
            .field(
                "intmtx_ready",
                &format_args!("{}", self.intmtx_ready().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTMTX_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable intmtx clock"]
    #[inline(always)]
    #[must_use]
    pub fn intmtx_clk_en(&mut self) -> INTMTX_CLK_EN_W<INTMTX_CONF_SPEC, 0> {
        INTMTX_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset intmtx module"]
    #[inline(always)]
    #[must_use]
    pub fn intmtx_rst_en(&mut self) -> INTMTX_RST_EN_W<INTMTX_CONF_SPEC, 1> {
        INTMTX_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "INTMTX configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmtx_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmtx_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMTX_CONF_SPEC;
impl crate::RegisterSpec for INTMTX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmtx_conf::R`](R) reader structure"]
impl crate::Readable for INTMTX_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmtx_conf::W`](W) writer structure"]
impl crate::Writable for INTMTX_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTMTX_CONF to value 0x05"]
impl crate::Resettable for INTMTX_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
