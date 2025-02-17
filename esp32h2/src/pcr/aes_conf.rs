#[doc = "Register `AES_CONF` reader"]
pub type R = crate::R<AES_CONF_SPEC>;
#[doc = "Register `AES_CONF` writer"]
pub type W = crate::W<AES_CONF_SPEC>;
#[doc = "Field `AES_CLK_EN` reader - Set 1 to enable aes clock"]
pub type AES_CLK_EN_R = crate::BitReader;
#[doc = "Field `AES_CLK_EN` writer - Set 1 to enable aes clock"]
pub type AES_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AES_RST_EN` reader - Set 0 to reset aes module"]
pub type AES_RST_EN_R = crate::BitReader;
#[doc = "Field `AES_RST_EN` writer - Set 0 to reset aes module"]
pub type AES_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AES_READY` reader - Query this field after reset aes module"]
pub type AES_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable aes clock"]
    #[inline(always)]
    pub fn aes_clk_en(&self) -> AES_CLK_EN_R {
        AES_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset aes module"]
    #[inline(always)]
    pub fn aes_rst_en(&self) -> AES_RST_EN_R {
        AES_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset aes module"]
    #[inline(always)]
    pub fn aes_ready(&self) -> AES_READY_R {
        AES_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_CONF")
            .field("aes_clk_en", &format_args!("{}", self.aes_clk_en().bit()))
            .field("aes_rst_en", &format_args!("{}", self.aes_rst_en().bit()))
            .field("aes_ready", &format_args!("{}", self.aes_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AES_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable aes clock"]
    #[inline(always)]
    #[must_use]
    pub fn aes_clk_en(&mut self) -> AES_CLK_EN_W<AES_CONF_SPEC, 0> {
        AES_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset aes module"]
    #[inline(always)]
    #[must_use]
    pub fn aes_rst_en(&mut self) -> AES_RST_EN_W<AES_CONF_SPEC, 1> {
        AES_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_CONF_SPEC;
impl crate::RegisterSpec for AES_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_conf::R`](R) reader structure"]
impl crate::Readable for AES_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aes_conf::W`](W) writer structure"]
impl crate::Writable for AES_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_CONF to value 0x05"]
impl crate::Resettable for AES_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
