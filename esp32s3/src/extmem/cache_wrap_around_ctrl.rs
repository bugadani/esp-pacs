#[doc = "Register `CACHE_WRAP_AROUND_CTRL` reader"]
pub type R = crate::R<CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Register `CACHE_WRAP_AROUND_CTRL` writer"]
pub type W = crate::W<CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Field `CACHE_FLASH_WRAP_AROUND` reader - The bit is used to enable wrap around mode when read data from flash."]
pub type CACHE_FLASH_WRAP_AROUND_R = crate::BitReader;
#[doc = "Field `CACHE_FLASH_WRAP_AROUND` writer - The bit is used to enable wrap around mode when read data from flash."]
pub type CACHE_FLASH_WRAP_AROUND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CACHE_SRAM_RD_WRAP_AROUND` reader - The bit is used to enable wrap around mode when read data from spiram."]
pub type CACHE_SRAM_RD_WRAP_AROUND_R = crate::BitReader;
#[doc = "Field `CACHE_SRAM_RD_WRAP_AROUND` writer - The bit is used to enable wrap around mode when read data from spiram."]
pub type CACHE_SRAM_RD_WRAP_AROUND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable wrap around mode when read data from flash."]
    #[inline(always)]
    pub fn cache_flash_wrap_around(&self) -> CACHE_FLASH_WRAP_AROUND_R {
        CACHE_FLASH_WRAP_AROUND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable wrap around mode when read data from spiram."]
    #[inline(always)]
    pub fn cache_sram_rd_wrap_around(&self) -> CACHE_SRAM_RD_WRAP_AROUND_R {
        CACHE_SRAM_RD_WRAP_AROUND_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_WRAP_AROUND_CTRL")
            .field(
                "cache_flash_wrap_around",
                &format_args!("{}", self.cache_flash_wrap_around().bit()),
            )
            .field(
                "cache_sram_rd_wrap_around",
                &format_args!("{}", self.cache_sram_rd_wrap_around().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_WRAP_AROUND_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable wrap around mode when read data from flash."]
    #[inline(always)]
    #[must_use]
    pub fn cache_flash_wrap_around(
        &mut self,
    ) -> CACHE_FLASH_WRAP_AROUND_W<CACHE_WRAP_AROUND_CTRL_SPEC, 0> {
        CACHE_FLASH_WRAP_AROUND_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable wrap around mode when read data from spiram."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_rd_wrap_around(
        &mut self,
    ) -> CACHE_SRAM_RD_WRAP_AROUND_W<CACHE_WRAP_AROUND_CTRL_SPEC, 1> {
        CACHE_SRAM_RD_WRAP_AROUND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_wrap_around_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_wrap_around_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_WRAP_AROUND_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_WRAP_AROUND_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_wrap_around_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_WRAP_AROUND_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_wrap_around_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_WRAP_AROUND_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_WRAP_AROUND_CTRL to value 0"]
impl crate::Resettable for CACHE_WRAP_AROUND_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
