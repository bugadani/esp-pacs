#[doc = "Register `W8` reader"]
pub type R = crate::R<W8_SPEC>;
#[doc = "Register `W8` writer"]
pub type W = crate::W<W8_SPEC>;
#[doc = "Field `BUF8` reader - data buffer"]
pub type BUF8_R = crate::FieldReader<u32>;
#[doc = "Field `BUF8` writer - data buffer"]
pub type BUF8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf8(&self) -> BUF8_R {
        BUF8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W8")
            .field("buf8", &format_args!("{}", self.buf8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf8(&mut self) -> BUF8_W<W8_SPEC, 0> {
        BUF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI1 memory data buffer8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W8_SPEC;
impl crate::RegisterSpec for W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w8::R`](R) reader structure"]
impl crate::Readable for W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w8::W`](W) writer structure"]
impl crate::Writable for W8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W8 to value 0"]
impl crate::Resettable for W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
