#[doc = "Register `ICACHE_ATOMIC_OPERATE_ENA` reader"]
pub type R = crate::R<ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "Register `ICACHE_ATOMIC_OPERATE_ENA` writer"]
pub type W = crate::W<ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "Field `ICACHE_ATOMIC_OPERATE_ENA` reader - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type ICACHE_ATOMIC_OPERATE_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_ATOMIC_OPERATE_ENA` writer - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type ICACHE_ATOMIC_OPERATE_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn icache_atomic_operate_ena(&self) -> ICACHE_ATOMIC_OPERATE_ENA_R {
        ICACHE_ATOMIC_OPERATE_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_ATOMIC_OPERATE_ENA")
            .field(
                "icache_atomic_operate_ena",
                &format_args!("{}", self.icache_atomic_operate_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_ATOMIC_OPERATE_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    #[must_use]
    pub fn icache_atomic_operate_ena(
        &mut self,
    ) -> ICACHE_ATOMIC_OPERATE_ENA_W<ICACHE_ATOMIC_OPERATE_ENA_SPEC, 0> {
        ICACHE_ATOMIC_OPERATE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_atomic_operate_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_atomic_operate_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_ATOMIC_OPERATE_ENA_SPEC;
impl crate::RegisterSpec for ICACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_atomic_operate_ena::R`](R) reader structure"]
impl crate::Readable for ICACHE_ATOMIC_OPERATE_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_atomic_operate_ena::W`](W) writer structure"]
impl crate::Writable for ICACHE_ATOMIC_OPERATE_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_ATOMIC_OPERATE_ENA to value 0x01"]
impl crate::Resettable for ICACHE_ATOMIC_OPERATE_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
