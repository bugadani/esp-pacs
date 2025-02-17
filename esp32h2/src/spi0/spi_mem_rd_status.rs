#[doc = "Register `SPI_MEM_RD_STATUS` reader"]
pub type R = crate::R<SPI_MEM_RD_STATUS_SPEC>;
#[doc = "Register `SPI_MEM_RD_STATUS` writer"]
pub type W = crate::W<SPI_MEM_RD_STATUS_SPEC>;
#[doc = "Field `SPI_MEM_WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_mem_wb_mode(&self) -> SPI_MEM_WB_MODE_R {
        SPI_MEM_WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_RD_STATUS")
            .field(
                "spi_mem_wb_mode",
                &format_args!("{}", self.spi_mem_wb_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_RD_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wb_mode(&mut self) -> SPI_MEM_WB_MODE_W<SPI_MEM_RD_STATUS_SPEC, 16> {
        SPI_MEM_WB_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI0 read control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_rd_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_rd_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_RD_STATUS_SPEC;
impl crate::RegisterSpec for SPI_MEM_RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_rd_status::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_rd_status::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_RD_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_RD_STATUS to value 0"]
impl crate::Resettable for SPI_MEM_RD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
