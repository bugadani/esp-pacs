#[doc = "Register `DMA_APBPERI_PMS_MONITOR_0` reader"]
pub type R = crate::R<DMA_APBPERI_PMS_MONITOR_0_SPEC>;
#[doc = "Register `DMA_APBPERI_PMS_MONITOR_0` writer"]
pub type W = crate::W<DMA_APBPERI_PMS_MONITOR_0_SPEC>;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_LOCK` reader - dma_apbperi_pms_monitor_lock"]
pub type DMA_APBPERI_PMS_MONITOR_LOCK_R = crate::BitReader;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_LOCK` writer - dma_apbperi_pms_monitor_lock"]
pub type DMA_APBPERI_PMS_MONITOR_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - dma_apbperi_pms_monitor_lock"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_lock(&self) -> DMA_APBPERI_PMS_MONITOR_LOCK_R {
        DMA_APBPERI_PMS_MONITOR_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_PMS_MONITOR_0")
            .field(
                "dma_apbperi_pms_monitor_lock",
                &format_args!("{}", self.dma_apbperi_pms_monitor_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_APBPERI_PMS_MONITOR_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - dma_apbperi_pms_monitor_lock"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_pms_monitor_lock(
        &mut self,
    ) -> DMA_APBPERI_PMS_MONITOR_LOCK_W<DMA_APBPERI_PMS_MONITOR_0_SPEC, 0> {
        DMA_APBPERI_PMS_MONITOR_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_pms_monitor_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_APBPERI_PMS_MONITOR_0_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_apbperi_pms_monitor_0::R`](R) reader structure"]
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_apbperi_pms_monitor_0::W`](W) writer structure"]
impl crate::Writable for DMA_APBPERI_PMS_MONITOR_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_APBPERI_PMS_MONITOR_0 to value 0"]
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
