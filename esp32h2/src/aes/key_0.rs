#[doc = "Register `KEY_0` reader"]
pub type R = crate::R<KEY_0_SPEC>;
#[doc = "Register `KEY_0` writer"]
pub type W = crate::W<KEY_0_SPEC>;
#[doc = "Field `KEY_0` reader - This bits stores key_0 that is a part of key material."]
pub type KEY_0_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_0` writer - This bits stores key_0 that is a part of key material."]
pub type KEY_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_0 that is a part of key material."]
    #[inline(always)]
    pub fn key_0(&self) -> KEY_0_R {
        KEY_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_0")
            .field("key_0", &format_args!("{}", self.key_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_0 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_0(&mut self) -> KEY_0_W<KEY_0_SPEC, 0> {
        KEY_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Key material key_0 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_0_SPEC;
impl crate::RegisterSpec for KEY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_0::R`](R) reader structure"]
impl crate::Readable for KEY_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_0::W`](W) writer structure"]
impl crate::Writable for KEY_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_0 to value 0"]
impl crate::Resettable for KEY_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
