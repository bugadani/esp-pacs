#[doc = "Register `T_LENGTH` reader"]
pub type R = crate::R<T_LENGTH_SPEC>;
#[doc = "Register `T_LENGTH` writer"]
pub type W = crate::W<T_LENGTH_SPEC>;
#[doc = "Field `T_LENGTH` reader - Sha t_length (used if and only if mode == SHA_512/t)."]
pub type T_LENGTH_R = crate::FieldReader;
#[doc = "Field `T_LENGTH` writer - Sha t_length (used if and only if mode == SHA_512/t)."]
pub type T_LENGTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Sha t_length (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    pub fn t_length(&self) -> T_LENGTH_R {
        T_LENGTH_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T_LENGTH")
            .field("t_length", &format_args!("{}", self.t_length().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T_LENGTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sha t_length (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    #[must_use]
    pub fn t_length(&mut self) -> T_LENGTH_W<T_LENGTH_SPEC, 0> {
        T_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SHA 512/t configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_LENGTH_SPEC;
impl crate::RegisterSpec for T_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_length::R`](R) reader structure"]
impl crate::Readable for T_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_length::W`](W) writer structure"]
impl crate::Writable for T_LENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T_LENGTH to value 0"]
impl crate::Resettable for T_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
