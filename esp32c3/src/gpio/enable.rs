#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `DATA` reader - GPIO output enable register for GPIO0-25"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - GPIO output enable register for GPIO0-25"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
impl R {
    #[doc = "Bits 0:25 - GPIO output enable register for GPIO0-25"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:25 - GPIO output enable register for GPIO0-25"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<ENABLE_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO output enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
