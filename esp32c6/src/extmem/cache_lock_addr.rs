#[doc = "Register `CACHE_LOCK_ADDR` reader"]
pub type R = crate::R<CACHE_LOCK_ADDR_SPEC>;
#[doc = "Register `CACHE_LOCK_ADDR` writer"]
pub type W = crate::W<CACHE_LOCK_ADDR_SPEC>;
#[doc = "Field `CACHE_LOCK_ADDR` reader - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
pub type CACHE_LOCK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_LOCK_ADDR` writer - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
pub type CACHE_LOCK_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
    #[inline(always)]
    pub fn cache_lock_addr(&self) -> CACHE_LOCK_ADDR_R {
        CACHE_LOCK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_LOCK_ADDR")
            .field(
                "cache_lock_addr",
                &format_args!("{}", self.cache_lock_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_LOCK_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
    #[inline(always)]
    #[must_use]
    pub fn cache_lock_addr(&mut self) -> CACHE_LOCK_ADDR_W<CACHE_LOCK_ADDR_SPEC, 0> {
        CACHE_LOCK_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Lock (manual lock) address configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_lock_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_lock_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_LOCK_ADDR_SPEC;
impl crate::RegisterSpec for CACHE_LOCK_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_lock_addr::R`](R) reader structure"]
impl crate::Readable for CACHE_LOCK_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_lock_addr::W`](W) writer structure"]
impl crate::Writable for CACHE_LOCK_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_LOCK_ADDR to value 0"]
impl crate::Resettable for CACHE_LOCK_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
