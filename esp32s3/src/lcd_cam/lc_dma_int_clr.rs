#[doc = "Register `LC_DMA_INT_CLR` writer"]
pub type W = crate::W<LC_DMA_INT_CLR_SPEC>;
#[doc = "Field `LCD_VSYNC_INT_CLR` writer - The clear bit for LCD frame end interrupt."]
pub type LCD_VSYNC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCD_TRANS_DONE_INT_CLR` writer - The clear bit for lcd transfer end interrupt."]
pub type LCD_TRANS_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAM_VSYNC_INT_CLR` writer - The clear bit for Camera frame end interrupt."]
pub type CAM_VSYNC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAM_HS_INT_CLR` writer - The clear bit for Camera line interrupt."]
pub type CAM_HS_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LC_DMA_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for LCD frame end interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vsync_int_clr(&mut self) -> LCD_VSYNC_INT_CLR_W<LC_DMA_INT_CLR_SPEC, 0> {
        LCD_VSYNC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The clear bit for lcd transfer end interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_trans_done_int_clr(&mut self) -> LCD_TRANS_DONE_INT_CLR_W<LC_DMA_INT_CLR_SPEC, 1> {
        LCD_TRANS_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The clear bit for Camera frame end interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cam_vsync_int_clr(&mut self) -> CAM_VSYNC_INT_CLR_W<LC_DMA_INT_CLR_SPEC, 2> {
        CAM_VSYNC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The clear bit for Camera line interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cam_hs_int_clr(&mut self) -> CAM_HS_INT_CLR_W<LC_DMA_INT_CLR_SPEC, 3> {
        CAM_HS_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LCD_camera DMA inturrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_dma_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for LC_DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lc_dma_int_clr::W`](W) writer structure"]
impl crate::Writable for LC_DMA_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LC_DMA_INT_CLR to value 0"]
impl crate::Resettable for LC_DMA_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
