#[doc = "Register `PRO_CACHE_ACS_CNT_CLR` writer"]
pub type W = crate::W<PRO_CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "Field `PRO_DCACHE_ACS_CNT_CLR` writer - The bit is used to clear dcache counter which include DC_PRELOAD_CNT_REG, DC_PRELOAD_EVICT_CNT_REG, DC_PRELOAD_MISS_CNT_REG, DBUS0-2_ABANDON_CNT_REG, DBUS0-2_ACS_WB_CNT_REG, DBUS0-2_ACS_MISS_CNT_REG and DBUS0-2_ACS_CNT_REG."]
pub type PRO_DCACHE_ACS_CNT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_ICACHE_ACS_CNT_CLR` writer - The bit is used to clear icache counter which include IC_PRELOAD_CNT_REG, IC_PRELOAD_MISS_CNT_REG, IBUS0-2_ABANDON_CNT_REG, IBUS0-2_ACS_MISS_CNT_REG and IBUS0-2_ACS_CNT_REG."]
pub type PRO_ICACHE_ACS_CNT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_ACS_CNT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear dcache counter which include DC_PRELOAD_CNT_REG, DC_PRELOAD_EVICT_CNT_REG, DC_PRELOAD_MISS_CNT_REG, DBUS0-2_ABANDON_CNT_REG, DBUS0-2_ACS_WB_CNT_REG, DBUS0-2_ACS_MISS_CNT_REG and DBUS0-2_ACS_CNT_REG."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_acs_cnt_clr(
        &mut self,
    ) -> PRO_DCACHE_ACS_CNT_CLR_W<PRO_CACHE_ACS_CNT_CLR_SPEC, 0> {
        PRO_DCACHE_ACS_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear icache counter which include IC_PRELOAD_CNT_REG, IC_PRELOAD_MISS_CNT_REG, IBUS0-2_ABANDON_CNT_REG, IBUS0-2_ACS_MISS_CNT_REG and IBUS0-2_ACS_CNT_REG."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_acs_cnt_clr(
        &mut self,
    ) -> PRO_ICACHE_ACS_CNT_CLR_W<PRO_CACHE_ACS_CNT_CLR_SPEC, 1> {
        PRO_ICACHE_ACS_CNT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_acs_cnt_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_ACS_CNT_CLR_SPEC;
impl crate::RegisterSpec for PRO_CACHE_ACS_CNT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pro_cache_acs_cnt_clr::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_ACS_CNT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_ACS_CNT_CLR to value 0"]
impl crate::Resettable for PRO_CACHE_ACS_CNT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
