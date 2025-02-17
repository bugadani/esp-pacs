#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_RAW` reader"]
pub type R = crate::R<L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_RAW` writer"]
pub type W = crate::W<L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Field `L2_CACHE_PLD_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when L2-Cache preload-operation is done."]
pub type L2_CACHE_PLD_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PLD_DONE_INT_RAW` writer - The raw bit of the interrupt that occurs only when L2-Cache preload-operation is done."]
pub type L2_CACHE_PLD_DONE_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when L2-Cache preload-operation error occurs."]
pub type L2_CACHE_PLD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_RAW` writer - The raw bit of the interrupt that occurs only when L2-Cache preload-operation error occurs."]
pub type L2_CACHE_PLD_ERR_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - The raw bit of the interrupt that occurs only when L2-Cache preload-operation is done."]
    #[inline(always)]
    pub fn l2_cache_pld_done_int_raw(&self) -> L2_CACHE_PLD_DONE_INT_RAW_R {
        L2_CACHE_PLD_DONE_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt that occurs only when L2-Cache preload-operation error occurs."]
    #[inline(always)]
    pub fn l2_cache_pld_err_int_raw(&self) -> L2_CACHE_PLD_ERR_INT_RAW_R {
        L2_CACHE_PLD_ERR_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_SYNC_PRELOAD_INT_RAW")
            .field(
                "l2_cache_pld_done_int_raw",
                &format_args!("{}", self.l2_cache_pld_done_int_raw().bit()),
            )
            .field(
                "l2_cache_pld_err_int_raw",
                &format_args!("{}", self.l2_cache_pld_err_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 5 - The raw bit of the interrupt that occurs only when L2-Cache preload-operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_pld_done_int_raw(
        &mut self,
    ) -> L2_CACHE_PLD_DONE_INT_RAW_W<L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC, 5> {
        L2_CACHE_PLD_DONE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt that occurs only when L2-Cache preload-operation error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_pld_err_int_raw(
        &mut self,
    ) -> L2_CACHE_PLD_ERR_INT_RAW_W<L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC, 12> {
        L2_CACHE_PLD_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sync Preload operation Interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_sync_preload_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_sync_preload_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC;
impl crate::RegisterSpec for L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_sync_preload_int_raw::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_sync_preload_int_raw::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2_CACHE_SYNC_PRELOAD_INT_RAW to value 0"]
impl crate::Resettable for L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
