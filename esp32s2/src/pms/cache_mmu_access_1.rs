#[doc = "Register `CACHE_MMU_ACCESS_1` reader"]
pub type R = crate::R<CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "Register `CACHE_MMU_ACCESS_1` writer"]
pub type W = crate::W<CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "Field `PRO_MMU_RD_ACS` reader - Setting to 1 permits read access to MMU memory."]
pub type PRO_MMU_RD_ACS_R = crate::BitReader;
#[doc = "Field `PRO_MMU_RD_ACS` writer - Setting to 1 permits read access to MMU memory."]
pub type PRO_MMU_RD_ACS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_MMU_WR_ACS` reader - Setting to 1 permits write access to MMU memory."]
pub type PRO_MMU_WR_ACS_R = crate::BitReader;
#[doc = "Field `PRO_MMU_WR_ACS` writer - Setting to 1 permits write access to MMU memory."]
pub type PRO_MMU_WR_ACS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Setting to 1 permits read access to MMU memory."]
    #[inline(always)]
    pub fn pro_mmu_rd_acs(&self) -> PRO_MMU_RD_ACS_R {
        PRO_MMU_RD_ACS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 permits write access to MMU memory."]
    #[inline(always)]
    pub fn pro_mmu_wr_acs(&self) -> PRO_MMU_WR_ACS_R {
        PRO_MMU_WR_ACS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_ACCESS_1")
            .field(
                "pro_mmu_rd_acs",
                &format_args!("{}", self.pro_mmu_rd_acs().bit()),
            )
            .field(
                "pro_mmu_wr_acs",
                &format_args!("{}", self.pro_mmu_wr_acs().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MMU_ACCESS_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 permits read access to MMU memory."]
    #[inline(always)]
    #[must_use]
    pub fn pro_mmu_rd_acs(&mut self) -> PRO_MMU_RD_ACS_W<CACHE_MMU_ACCESS_1_SPEC, 0> {
        PRO_MMU_RD_ACS_W::new(self)
    }
    #[doc = "Bit 1 - Setting to 1 permits write access to MMU memory."]
    #[inline(always)]
    #[must_use]
    pub fn pro_mmu_wr_acs(&mut self) -> PRO_MMU_WR_ACS_W<CACHE_MMU_ACCESS_1_SPEC, 1> {
        PRO_MMU_WR_ACS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache MMU permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_access_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_access_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MMU_ACCESS_1_SPEC;
impl crate::RegisterSpec for CACHE_MMU_ACCESS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_mmu_access_1::R`](R) reader structure"]
impl crate::Readable for CACHE_MMU_ACCESS_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_mmu_access_1::W`](W) writer structure"]
impl crate::Writable for CACHE_MMU_ACCESS_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_MMU_ACCESS_1 to value 0x03"]
impl crate::Resettable for CACHE_MMU_ACCESS_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
