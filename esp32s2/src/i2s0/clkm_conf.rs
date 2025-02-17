#[doc = "Register `CLKM_CONF` reader"]
pub type R = crate::R<CLKM_CONF_SPEC>;
#[doc = "Register `CLKM_CONF` writer"]
pub type W = crate::W<CLKM_CONF_SPEC>;
#[doc = "Field `CLKM_DIV_NUM` reader - Integral I2S clock divider value."]
pub type CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CLKM_DIV_NUM` writer - Integral I2S clock divider value."]
pub type CLKM_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLKM_DIV_B` reader - Fractional clock divider numerator value."]
pub type CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `CLKM_DIV_B` writer - Fractional clock divider numerator value."]
pub type CLKM_DIV_B_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CLKM_DIV_A` reader - Fractional clock divider denominator value."]
pub type CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `CLKM_DIV_A` writer - Fractional clock divider denominator value."]
pub type CLKM_DIV_A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clock gate."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clock gate."]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_SEL` reader - Set this bit to select I2S module clock source. 0: No clock. 1: APLL_CLK. 2: PLL_160M_CLK. 3: No clock."]
pub type CLK_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_SEL` writer - Set this bit to select I2S module clock source. 0: No clock. 1: APLL_CLK. 2: PLL_160M_CLK. 3: No clock."]
pub type CLK_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:7 - Integral I2S clock divider value."]
    #[inline(always)]
    pub fn clkm_div_num(&self) -> CLKM_DIV_NUM_R {
        CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Fractional clock divider numerator value."]
    #[inline(always)]
    pub fn clkm_div_b(&self) -> CLKM_DIV_B_R {
        CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - Fractional clock divider denominator value."]
    #[inline(always)]
    pub fn clkm_div_a(&self) -> CLKM_DIV_A_R {
        CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - Set this bit to enable clock gate."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Set this bit to select I2S module clock source. 0: No clock. 1: APLL_CLK. 2: PLL_160M_CLK. 3: No clock."]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKM_CONF")
            .field(
                "clkm_div_num",
                &format_args!("{}", self.clkm_div_num().bits()),
            )
            .field("clkm_div_b", &format_args!("{}", self.clkm_div_b().bits()))
            .field("clkm_div_a", &format_args!("{}", self.clkm_div_a().bits()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field("clk_sel", &format_args!("{}", self.clk_sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S clock divider value."]
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_num(&mut self) -> CLKM_DIV_NUM_W<CLKM_CONF_SPEC, 0> {
        CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:13 - Fractional clock divider numerator value."]
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_b(&mut self) -> CLKM_DIV_B_W<CLKM_CONF_SPEC, 8> {
        CLKM_DIV_B_W::new(self)
    }
    #[doc = "Bits 14:19 - Fractional clock divider denominator value."]
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_a(&mut self) -> CLKM_DIV_A_W<CLKM_CONF_SPEC, 14> {
        CLKM_DIV_A_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to enable clock gate."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLKM_CONF_SPEC, 20> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 21:22 - Set this bit to select I2S module clock source. 0: No clock. 1: APLL_CLK. 2: PLL_160M_CLK. 3: No clock."]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<CLKM_CONF_SPEC, 21> {
        CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S module clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKM_CONF_SPEC;
impl crate::RegisterSpec for CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkm_conf::R`](R) reader structure"]
impl crate::Readable for CLKM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkm_conf::W`](W) writer structure"]
impl crate::Writable for CLKM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKM_CONF to value 0x04"]
impl crate::Resettable for CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
