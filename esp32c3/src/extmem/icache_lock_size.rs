#[doc = "Register `ICACHE_LOCK_SIZE` reader"]
pub type R = crate::R<ICACHE_LOCK_SIZE_SPEC>;
#[doc = "Register `ICACHE_LOCK_SIZE` writer"]
pub type W = crate::W<ICACHE_LOCK_SIZE_SPEC>;
#[doc = "Field `ICACHE_LOCK_SIZE` reader - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with ICACHE_LOCK_ADDR_REG."]
pub type ICACHE_LOCK_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `ICACHE_LOCK_SIZE` writer - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with ICACHE_LOCK_ADDR_REG."]
pub type ICACHE_LOCK_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with ICACHE_LOCK_ADDR_REG."]
    #[inline(always)]
    pub fn icache_lock_size(&self) -> ICACHE_LOCK_SIZE_R {
        ICACHE_LOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_LOCK_SIZE")
            .field(
                "icache_lock_size",
                &format_args!("{}", self.icache_lock_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_LOCK_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with ICACHE_LOCK_ADDR_REG."]
    #[inline(always)]
    #[must_use]
    pub fn icache_lock_size(&mut self) -> ICACHE_LOCK_SIZE_W<ICACHE_LOCK_SIZE_SPEC, 0> {
        ICACHE_LOCK_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_lock_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_lock_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_LOCK_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE_LOCK_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_lock_size::R`](R) reader structure"]
impl crate::Readable for ICACHE_LOCK_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_lock_size::W`](W) writer structure"]
impl crate::Writable for ICACHE_LOCK_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_LOCK_SIZE to value 0"]
impl crate::Resettable for ICACHE_LOCK_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
