#[doc = "Register `BLOCK_MODE` reader"]
pub type R = crate::R<BLOCK_MODE_SPEC>;
#[doc = "Register `BLOCK_MODE` writer"]
pub type W = crate::W<BLOCK_MODE_SPEC>;
#[doc = "Field `BLOCK_MODE` reader - Defines the block cipher mode of the AES accelerator operating under the DMA-AES working mode. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: reserved, 0x7: reserved."]
pub type BLOCK_MODE_R = crate::FieldReader;
#[doc = "Field `BLOCK_MODE` writer - Defines the block cipher mode of the AES accelerator operating under the DMA-AES working mode. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: reserved, 0x7: reserved."]
pub type BLOCK_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Defines the block cipher mode of the AES accelerator operating under the DMA-AES working mode. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: reserved, 0x7: reserved."]
    #[inline(always)]
    pub fn block_mode(&self) -> BLOCK_MODE_R {
        BLOCK_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLOCK_MODE")
            .field("block_mode", &format_args!("{}", self.block_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLOCK_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the block cipher mode of the AES accelerator operating under the DMA-AES working mode. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: reserved, 0x7: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn block_mode(&mut self) -> BLOCK_MODE_W<BLOCK_MODE_SPEC, 0> {
        BLOCK_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES cipher block mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCK_MODE_SPEC;
impl crate::RegisterSpec for BLOCK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block_mode::R`](R) reader structure"]
impl crate::Readable for BLOCK_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`block_mode::W`](W) writer structure"]
impl crate::Writable for BLOCK_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCK_MODE to value 0"]
impl crate::Resettable for BLOCK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
