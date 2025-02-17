#[doc = "Register `SRAM_DRD_CMD` reader"]
pub type R = crate::R<SRAM_DRD_CMD_SPEC>;
#[doc = "Register `SRAM_DRD_CMD` writer"]
pub type W = crate::W<SRAM_DRD_CMD_SPEC>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_VALUE` reader - For SPI0 When cache mode is enable it is the read command value of command phase for SRAM."]
pub type CACHE_SRAM_USR_RD_CMD_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_VALUE` writer - For SPI0 When cache mode is enable it is the read command value of command phase for SRAM."]
pub type CACHE_SRAM_USR_RD_CMD_VALUE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_BITLEN` reader - For SPI0 When cache mode is enable it is the length in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
pub type CACHE_SRAM_USR_RD_CMD_BITLEN_R = crate::FieldReader;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_BITLEN` writer - For SPI0 When cache mode is enable it is the length in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
pub type CACHE_SRAM_USR_RD_CMD_BITLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:15 - For SPI0 When cache mode is enable it is the read command value of command phase for SRAM."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_value(&self) -> CACHE_SRAM_USR_RD_CMD_VALUE_R {
        CACHE_SRAM_USR_RD_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - For SPI0 When cache mode is enable it is the length in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_bitlen(&self) -> CACHE_SRAM_USR_RD_CMD_BITLEN_R {
        CACHE_SRAM_USR_RD_CMD_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_DRD_CMD")
            .field(
                "cache_sram_usr_rd_cmd_value",
                &format_args!("{}", self.cache_sram_usr_rd_cmd_value().bits()),
            )
            .field(
                "cache_sram_usr_rd_cmd_bitlen",
                &format_args!("{}", self.cache_sram_usr_rd_cmd_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_DRD_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - For SPI0 When cache mode is enable it is the read command value of command phase for SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_rd_cmd_value(
        &mut self,
    ) -> CACHE_SRAM_USR_RD_CMD_VALUE_W<SRAM_DRD_CMD_SPEC, 0> {
        CACHE_SRAM_USR_RD_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 28:31 - For SPI0 When cache mode is enable it is the length in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_rd_cmd_bitlen(
        &mut self,
    ) -> CACHE_SRAM_USR_RD_CMD_BITLEN_W<SRAM_DRD_CMD_SPEC, 28> {
        CACHE_SRAM_USR_RD_CMD_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_drd_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_drd_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_DRD_CMD_SPEC;
impl crate::RegisterSpec for SRAM_DRD_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_drd_cmd::R`](R) reader structure"]
impl crate::Readable for SRAM_DRD_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_drd_cmd::W`](W) writer structure"]
impl crate::Writable for SRAM_DRD_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_DRD_CMD to value 0"]
impl crate::Resettable for SRAM_DRD_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
