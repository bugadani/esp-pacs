#[doc = "Register `L1_CACHE_AUTOLOAD_BUF_CLR_CTRL` reader"]
pub type R = crate::R<L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Register `L1_CACHE_AUTOLOAD_BUF_CLR_CTRL` writer"]
pub type W = crate::W<L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE0_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L1-ICache0. If this bit is active, autoload will not work in L1-ICache0. This bit should not be active when autoload works in L1-ICache0."]
pub type L1_ICACHE0_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L1-ICache1. If this bit is active, autoload will not work in L1-ICache1. This bit should not be active when autoload works in L1-ICache1."]
pub type L1_ICACHE1_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_ALD_BUF_CLR` reader - Reserved"]
pub type L1_ICACHE2_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_ALD_BUF_CLR` reader - Reserved"]
pub type L1_ICACHE3_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `L1_CACHE_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L1-Cache. If this bit is active, autoload will not work in L1-Cache. This bit should not be active when autoload works in L1-Cache."]
pub type L1_CACHE_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `L1_CACHE_ALD_BUF_CLR` writer - set this bit to clear autoload-buffer inside L1-Cache. If this bit is active, autoload will not work in L1-Cache. This bit should not be active when autoload works in L1-Cache."]
pub type L1_CACHE_ALD_BUF_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - set this bit to clear autoload-buffer inside L1-ICache0. If this bit is active, autoload will not work in L1-ICache0. This bit should not be active when autoload works in L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_ald_buf_clr(&self) -> L1_ICACHE0_ALD_BUF_CLR_R {
        L1_ICACHE0_ALD_BUF_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit to clear autoload-buffer inside L1-ICache1. If this bit is active, autoload will not work in L1-ICache1. This bit should not be active when autoload works in L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_ald_buf_clr(&self) -> L1_ICACHE1_ALD_BUF_CLR_R {
        L1_ICACHE1_ALD_BUF_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_ald_buf_clr(&self) -> L1_ICACHE2_ALD_BUF_CLR_R {
        L1_ICACHE2_ALD_BUF_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_ald_buf_clr(&self) -> L1_ICACHE3_ALD_BUF_CLR_R {
        L1_ICACHE3_ALD_BUF_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to clear autoload-buffer inside L1-Cache. If this bit is active, autoload will not work in L1-Cache. This bit should not be active when autoload works in L1-Cache."]
    #[inline(always)]
    pub fn l1_cache_ald_buf_clr(&self) -> L1_CACHE_ALD_BUF_CLR_R {
        L1_CACHE_ALD_BUF_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_AUTOLOAD_BUF_CLR_CTRL")
            .field(
                "l1_icache0_ald_buf_clr",
                &format_args!("{}", self.l1_icache0_ald_buf_clr().bit()),
            )
            .field(
                "l1_icache1_ald_buf_clr",
                &format_args!("{}", self.l1_icache1_ald_buf_clr().bit()),
            )
            .field(
                "l1_icache2_ald_buf_clr",
                &format_args!("{}", self.l1_icache2_ald_buf_clr().bit()),
            )
            .field(
                "l1_icache3_ald_buf_clr",
                &format_args!("{}", self.l1_icache3_ald_buf_clr().bit()),
            )
            .field(
                "l1_cache_ald_buf_clr",
                &format_args!("{}", self.l1_cache_ald_buf_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - set this bit to clear autoload-buffer inside L1-Cache. If this bit is active, autoload will not work in L1-Cache. This bit should not be active when autoload works in L1-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_ald_buf_clr(
        &mut self,
    ) -> L1_CACHE_ALD_BUF_CLR_W<L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC, 4> {
        L1_CACHE_ALD_BUF_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache Autoload buffer clear control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_autoload_buf_clr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_autoload_buf_clr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_autoload_buf_clr_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_autoload_buf_clr_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_AUTOLOAD_BUF_CLR_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
