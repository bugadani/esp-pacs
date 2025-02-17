#[doc = "Register `BLOCK_NUM` reader"]
pub type R = crate::R<BLOCK_NUM_SPEC>;
#[doc = "Register `BLOCK_NUM` writer"]
pub type W = crate::W<BLOCK_NUM_SPEC>;
#[doc = "Field `BLOCK_NUM` reader - Stores the Block Number of plaintext or ciphertext when the AES accelerator operates under the DMA-AES working mode."]
pub type BLOCK_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK_NUM` writer - Stores the Block Number of plaintext or ciphertext when the AES accelerator operates under the DMA-AES working mode."]
pub type BLOCK_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the Block Number of plaintext or ciphertext when the AES accelerator operates under the DMA-AES working mode."]
    #[inline(always)]
    pub fn block_num(&self) -> BLOCK_NUM_R {
        BLOCK_NUM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLOCK_NUM")
            .field("block_num", &format_args!("{}", self.block_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLOCK_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the Block Number of plaintext or ciphertext when the AES accelerator operates under the DMA-AES working mode."]
    #[inline(always)]
    #[must_use]
    pub fn block_num(&mut self) -> BLOCK_NUM_W<BLOCK_NUM_SPEC, 0> {
        BLOCK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES block number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCK_NUM_SPEC;
impl crate::RegisterSpec for BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block_num::R`](R) reader structure"]
impl crate::Readable for BLOCK_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`block_num::W`](W) writer structure"]
impl crate::Writable for BLOCK_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCK_NUM to value 0"]
impl crate::Resettable for BLOCK_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
