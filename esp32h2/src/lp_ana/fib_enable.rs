#[doc = "Register `FIB_ENABLE` reader"]
pub type R = crate::R<FIB_ENABLE_SPEC>;
#[doc = "Register `FIB_ENABLE` writer"]
pub type W = crate::W<FIB_ENABLE_SPEC>;
#[doc = "Field `ANA_FIB_ENA` reader - need_des"]
pub type ANA_FIB_ENA_R = crate::FieldReader<u32>;
#[doc = "Field `ANA_FIB_ENA` writer - need_des"]
pub type ANA_FIB_ENA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn ana_fib_ena(&self) -> ANA_FIB_ENA_R {
        ANA_FIB_ENA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIB_ENABLE")
            .field(
                "ana_fib_ena",
                &format_args!("{}", self.ana_fib_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIB_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_fib_ena(&mut self) -> ANA_FIB_ENA_W<FIB_ENABLE_SPEC, 0> {
        ANA_FIB_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fib_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fib_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIB_ENABLE_SPEC;
impl crate::RegisterSpec for FIB_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fib_enable::R`](R) reader structure"]
impl crate::Readable for FIB_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fib_enable::W`](W) writer structure"]
impl crate::Writable for FIB_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIB_ENABLE to value 0xffff_ffff"]
impl crate::Resettable for FIB_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
