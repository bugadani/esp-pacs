#[doc = "Register `UART0_CONF` reader"]
pub type R = crate::R<UART0_CONF_SPEC>;
#[doc = "Register `UART0_CONF` writer"]
pub type W = crate::W<UART0_CONF_SPEC>;
#[doc = "Field `UART0_CLK_EN` reader - Set 1 to enable uart0 apb clock"]
pub type UART0_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART0_CLK_EN` writer - Set 1 to enable uart0 apb clock"]
pub type UART0_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART0_RST_EN` reader - Set 0 to reset uart0 module"]
pub type UART0_RST_EN_R = crate::BitReader;
#[doc = "Field `UART0_RST_EN` writer - Set 0 to reset uart0 module"]
pub type UART0_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable uart0 apb clock"]
    #[inline(always)]
    pub fn uart0_clk_en(&self) -> UART0_CLK_EN_R {
        UART0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset uart0 module"]
    #[inline(always)]
    pub fn uart0_rst_en(&self) -> UART0_RST_EN_R {
        UART0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0_CONF")
            .field(
                "uart0_clk_en",
                &format_args!("{}", self.uart0_clk_en().bit()),
            )
            .field(
                "uart0_rst_en",
                &format_args!("{}", self.uart0_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART0_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable uart0 apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart0_clk_en(&mut self) -> UART0_CLK_EN_W<UART0_CONF_SPEC, 0> {
        UART0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset uart0 module"]
    #[inline(always)]
    #[must_use]
    pub fn uart0_rst_en(&mut self) -> UART0_RST_EN_W<UART0_CONF_SPEC, 1> {
        UART0_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART0_CONF_SPEC;
impl crate::RegisterSpec for UART0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_conf::R`](R) reader structure"]
impl crate::Readable for UART0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart0_conf::W`](W) writer structure"]
impl crate::Writable for UART0_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART0_CONF to value 0x01"]
impl crate::Resettable for UART0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
