#[doc = "Register `DMA_CONTINUE` writer"]
pub type W = crate::W<DMA_CONTINUE_SPEC>;
#[doc = "Field `DMA_CONTINUE` writer - Write 1 to continue DMA-SHA calculation."]
pub type DMA_CONTINUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to continue DMA-SHA calculation."]
    #[inline(always)]
    #[must_use]
    pub fn dma_continue(&mut self) -> DMA_CONTINUE_W<DMA_CONTINUE_SPEC, 0> {
        DMA_CONTINUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Continues SHA operation (only effective in DMA-SHA mode)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONTINUE_SPEC;
impl crate::RegisterSpec for DMA_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_continue::W`](W) writer structure"]
impl crate::Writable for DMA_CONTINUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CONTINUE to value 0"]
impl crate::Resettable for DMA_CONTINUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
