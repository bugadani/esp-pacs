#[doc = "Register `CACHE_MMU_OWNER` reader"]
pub type R = crate::R<CACHE_MMU_OWNER_SPEC>;
#[doc = "Register `CACHE_MMU_OWNER` writer"]
pub type W = crate::W<CACHE_MMU_OWNER_SPEC>;
#[doc = "Field `CACHE_MMU_OWNER` reader - The bits are used to specify the owner of MMU.bit0: icache, bit1: dcache, bit2: dma, bit3: reserved."]
pub type CACHE_MMU_OWNER_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_MMU_OWNER` writer - The bits are used to specify the owner of MMU.bit0: icache, bit1: dcache, bit2: dma, bit3: reserved."]
pub type CACHE_MMU_OWNER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - The bits are used to specify the owner of MMU.bit0: icache, bit1: dcache, bit2: dma, bit3: reserved."]
    #[inline(always)]
    pub fn cache_mmu_owner(&self) -> CACHE_MMU_OWNER_R {
        CACHE_MMU_OWNER_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_OWNER")
            .field(
                "cache_mmu_owner",
                &format_args!("{}", self.cache_mmu_owner().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MMU_OWNER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - The bits are used to specify the owner of MMU.bit0: icache, bit1: dcache, bit2: dma, bit3: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_owner(&mut self) -> CACHE_MMU_OWNER_W<CACHE_MMU_OWNER_SPEC, 0> {
        CACHE_MMU_OWNER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_owner::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_owner::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MMU_OWNER_SPEC;
impl crate::RegisterSpec for CACHE_MMU_OWNER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_mmu_owner::R`](R) reader structure"]
impl crate::Readable for CACHE_MMU_OWNER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_mmu_owner::W`](W) writer structure"]
impl crate::Writable for CACHE_MMU_OWNER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_MMU_OWNER to value 0"]
impl crate::Resettable for CACHE_MMU_OWNER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
