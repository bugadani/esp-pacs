#[doc = "Register `PRO_DCACHE_LOCK0_SIZE` reader"]
pub type R = crate::R<PRO_DCACHE_LOCK0_SIZE_SPEC>;
#[doc = "Register `PRO_DCACHE_LOCK0_SIZE` writer"]
pub type W = crate::W<PRO_DCACHE_LOCK0_SIZE_SPEC>;
#[doc = "Field `PRO_DCACHE_LOCK0_SIZE` reader - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
pub type PRO_DCACHE_LOCK0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_DCACHE_LOCK0_SIZE` writer - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
pub type PRO_DCACHE_LOCK0_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
    #[inline(always)]
    pub fn pro_dcache_lock0_size(&self) -> PRO_DCACHE_LOCK0_SIZE_R {
        PRO_DCACHE_LOCK0_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_LOCK0_SIZE")
            .field(
                "pro_dcache_lock0_size",
                &format_args!("{}", self.pro_dcache_lock0_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_LOCK0_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_lock0_size(
        &mut self,
    ) -> PRO_DCACHE_LOCK0_SIZE_W<PRO_DCACHE_LOCK0_SIZE_SPEC, 0> {
        PRO_DCACHE_LOCK0_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_lock0_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_lock0_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_LOCK0_SIZE_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_LOCK0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_lock0_size::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_LOCK0_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_lock0_size::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_LOCK0_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_LOCK0_SIZE to value 0"]
impl crate::Resettable for PRO_DCACHE_LOCK0_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
