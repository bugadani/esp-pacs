#[doc = "Register `REGCLK` reader"]
pub type R = crate::R<REGCLK_SPEC>;
#[doc = "Register `REGCLK` writer"]
pub type W = crate::W<REGCLK_SPEC>;
#[doc = "Field `CLK_EN` reader - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGCLK")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<REGCLK_SPEC, 31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer group clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGCLK_SPEC;
impl crate::RegisterSpec for REGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regclk::R`](R) reader structure"]
impl crate::Readable for REGCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regclk::W`](W) writer structure"]
impl crate::Writable for REGCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGCLK to value 0"]
impl crate::Resettable for REGCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
