#[doc = "Register `CPU_PERI_RST_EN` reader"]
pub type R = crate::R<CPU_PERI_RST_EN_SPEC>;
#[doc = "Register `CPU_PERI_RST_EN` writer"]
pub type W = crate::W<CPU_PERI_RST_EN_SPEC>;
#[doc = "Field `RST_EN_ASSIST_DEBUG` reader - reg_rst_en_assist_debug"]
pub type RST_EN_ASSIST_DEBUG_R = crate::BitReader;
#[doc = "Field `RST_EN_ASSIST_DEBUG` writer - reg_rst_en_assist_debug"]
pub type RST_EN_ASSIST_DEBUG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RST_EN_DEDICATED_GPIO` reader - reg_rst_en_dedicated_gpio"]
pub type RST_EN_DEDICATED_GPIO_R = crate::BitReader;
#[doc = "Field `RST_EN_DEDICATED_GPIO` writer - reg_rst_en_dedicated_gpio"]
pub type RST_EN_DEDICATED_GPIO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 6 - reg_rst_en_assist_debug"]
    #[inline(always)]
    pub fn rst_en_assist_debug(&self) -> RST_EN_ASSIST_DEBUG_R {
        RST_EN_ASSIST_DEBUG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_rst_en_dedicated_gpio"]
    #[inline(always)]
    pub fn rst_en_dedicated_gpio(&self) -> RST_EN_DEDICATED_GPIO_R {
        RST_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_RST_EN")
            .field(
                "rst_en_assist_debug",
                &format_args!("{}", self.rst_en_assist_debug().bit()),
            )
            .field(
                "rst_en_dedicated_gpio",
                &format_args!("{}", self.rst_en_dedicated_gpio().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_PERI_RST_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 6 - reg_rst_en_assist_debug"]
    #[inline(always)]
    #[must_use]
    pub fn rst_en_assist_debug(&mut self) -> RST_EN_ASSIST_DEBUG_W<CPU_PERI_RST_EN_SPEC, 6> {
        RST_EN_ASSIST_DEBUG_W::new(self)
    }
    #[doc = "Bit 7 - reg_rst_en_dedicated_gpio"]
    #[inline(always)]
    #[must_use]
    pub fn rst_en_dedicated_gpio(&mut self) -> RST_EN_DEDICATED_GPIO_W<CPU_PERI_RST_EN_SPEC, 7> {
        RST_EN_DEDICATED_GPIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "cpu_peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_rst_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_rst_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PERI_RST_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_peri_rst_en::R`](R) reader structure"]
impl crate::Readable for CPU_PERI_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_peri_rst_en::W`](W) writer structure"]
impl crate::Writable for CPU_PERI_RST_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_PERI_RST_EN to value 0xc0"]
impl crate::Resettable for CPU_PERI_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
