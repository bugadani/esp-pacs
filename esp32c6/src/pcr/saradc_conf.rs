#[doc = "Register `SARADC_CONF` reader"]
pub type R = crate::R<SARADC_CONF_SPEC>;
#[doc = "Register `SARADC_CONF` writer"]
pub type W = crate::W<SARADC_CONF_SPEC>;
#[doc = "Field `SARADC_CLK_EN` reader - no use"]
pub type SARADC_CLK_EN_R = crate::BitReader;
#[doc = "Field `SARADC_CLK_EN` writer - no use"]
pub type SARADC_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARADC_RST_EN` reader - Set 0 to reset function_register of saradc module"]
pub type SARADC_RST_EN_R = crate::BitReader;
#[doc = "Field `SARADC_RST_EN` writer - Set 0 to reset function_register of saradc module"]
pub type SARADC_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARADC_REG_CLK_EN` reader - Set 1 to enable saradc apb clock"]
pub type SARADC_REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `SARADC_REG_CLK_EN` writer - Set 1 to enable saradc apb clock"]
pub type SARADC_REG_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARADC_REG_RST_EN` reader - Set 0 to reset apb_register of saradc module"]
pub type SARADC_REG_RST_EN_R = crate::BitReader;
#[doc = "Field `SARADC_REG_RST_EN` writer - Set 0 to reset apb_register of saradc module"]
pub type SARADC_REG_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - no use"]
    #[inline(always)]
    pub fn saradc_clk_en(&self) -> SARADC_CLK_EN_R {
        SARADC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset function_register of saradc module"]
    #[inline(always)]
    pub fn saradc_rst_en(&self) -> SARADC_RST_EN_R {
        SARADC_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable saradc apb clock"]
    #[inline(always)]
    pub fn saradc_reg_clk_en(&self) -> SARADC_REG_CLK_EN_R {
        SARADC_REG_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 0 to reset apb_register of saradc module"]
    #[inline(always)]
    pub fn saradc_reg_rst_en(&self) -> SARADC_REG_RST_EN_R {
        SARADC_REG_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SARADC_CONF")
            .field(
                "saradc_clk_en",
                &format_args!("{}", self.saradc_clk_en().bit()),
            )
            .field(
                "saradc_rst_en",
                &format_args!("{}", self.saradc_rst_en().bit()),
            )
            .field(
                "saradc_reg_clk_en",
                &format_args!("{}", self.saradc_reg_clk_en().bit()),
            )
            .field(
                "saradc_reg_rst_en",
                &format_args!("{}", self.saradc_reg_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SARADC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - no use"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_clk_en(&mut self) -> SARADC_CLK_EN_W<SARADC_CONF_SPEC, 0> {
        SARADC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset function_register of saradc module"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_rst_en(&mut self) -> SARADC_RST_EN_W<SARADC_CONF_SPEC, 1> {
        SARADC_RST_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable saradc apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_reg_clk_en(&mut self) -> SARADC_REG_CLK_EN_W<SARADC_CONF_SPEC, 2> {
        SARADC_REG_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set 0 to reset apb_register of saradc module"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_reg_rst_en(&mut self) -> SARADC_REG_RST_EN_W<SARADC_CONF_SPEC, 3> {
        SARADC_REG_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SARADC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SARADC_CONF_SPEC;
impl crate::RegisterSpec for SARADC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saradc_conf::R`](R) reader structure"]
impl crate::Readable for SARADC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saradc_conf::W`](W) writer structure"]
impl crate::Writable for SARADC_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SARADC_CONF to value 0x05"]
impl crate::Resettable for SARADC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
