#[doc = "Register `USER1` reader"]
pub type R = crate::R<USER1_SPEC>;
#[doc = "Register `USER1` writer"]
pub type W = crate::W<USER1_SPEC>;
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type USR_DUMMY_CYCLELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MST_WFULL_ERR_END_EN` reader - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type MST_WFULL_ERR_END_EN_R = crate::BitReader;
#[doc = "Field `MST_WFULL_ERR_END_EN` writer - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type MST_WFULL_ERR_END_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type CS_SETUP_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type CS_HOLD_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type USR_ADDR_BITLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&self) -> MST_WFULL_ERR_END_EN_R {
        MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER1")
            .field(
                "usr_dummy_cyclelen",
                &format_args!("{}", self.usr_dummy_cyclelen().bits()),
            )
            .field(
                "mst_wfull_err_end_en",
                &format_args!("{}", self.mst_wfull_err_end_en().bit()),
            )
            .field(
                "cs_setup_time",
                &format_args!("{}", self.cs_setup_time().bits()),
            )
            .field(
                "cs_hold_time",
                &format_args!("{}", self.cs_hold_time().bits()),
            )
            .field(
                "usr_addr_bitlen",
                &format_args!("{}", self.usr_addr_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W<USER1_SPEC, 0> {
        USR_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    #[must_use]
    pub fn mst_wfull_err_end_en(&mut self) -> MST_WFULL_ERR_END_EN_W<USER1_SPEC, 16> {
        MST_WFULL_ERR_END_EN_W::new(self)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<USER1_SPEC, 17> {
        CS_SETUP_TIME_W::new(self)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<USER1_SPEC, 22> {
        CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W<USER1_SPEC, 27> {
        USR_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI USER control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user1::R`](R) reader structure"]
impl crate::Readable for USER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`user1::W`](W) writer structure"]
impl crate::Writable for USER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER1 to value 0xb841_0007"]
impl crate::Resettable for USER1_SPEC {
    const RESET_VALUE: Self::Ux = 0xb841_0007;
}
