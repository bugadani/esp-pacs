#[doc = "Register `H_MEM%s` reader"]
pub type R = crate::R<H_MEM_SPEC>;
#[doc = "Register `H_MEM%s` writer"]
pub type W = crate::W<H_MEM_SPEC>;
#[doc = "Field `H` reader - Stores the %sth 32-bit piece of the Hash value."]
pub type H_R = crate::FieldReader<u32>;
#[doc = "Field `H` writer - Stores the %sth 32-bit piece of the Hash value."]
pub type H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the Hash value."]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("H_MEM")
            .field("h", &format_args!("{}", self.h().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<H_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the Hash value."]
    #[inline(always)]
    #[must_use]
    pub fn h(&mut self) -> H_W<H_MEM_SPEC, 0> {
        H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hash value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct H_MEM_SPEC;
impl crate::RegisterSpec for H_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_mem::R`](R) reader structure"]
impl crate::Readable for H_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`h_mem::W`](W) writer structure"]
impl crate::Writable for H_MEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets H_MEM%s to value 0"]
impl crate::Resettable for H_MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
