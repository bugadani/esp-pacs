#[doc = "Register `REMAINDER_BIT_NUM` reader"]
pub type R = crate::R<REMAINDER_BIT_NUM_SPEC>;
#[doc = "Register `REMAINDER_BIT_NUM` writer"]
pub type W = crate::W<REMAINDER_BIT_NUM_SPEC>;
#[doc = "Field `REMAINDER_BIT_NUM` reader - Those bits stores the number of remainder bit."]
pub type REMAINDER_BIT_NUM_R = crate::FieldReader;
#[doc = "Field `REMAINDER_BIT_NUM` writer - Those bits stores the number of remainder bit."]
pub type REMAINDER_BIT_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Those bits stores the number of remainder bit."]
    #[inline(always)]
    pub fn remainder_bit_num(&self) -> REMAINDER_BIT_NUM_R {
        REMAINDER_BIT_NUM_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAINDER_BIT_NUM")
            .field(
                "remainder_bit_num",
                &format_args!("{}", self.remainder_bit_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REMAINDER_BIT_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Those bits stores the number of remainder bit."]
    #[inline(always)]
    #[must_use]
    pub fn remainder_bit_num(&mut self) -> REMAINDER_BIT_NUM_W<REMAINDER_BIT_NUM_SPEC, 0> {
        REMAINDER_BIT_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES remainder bit number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remainder_bit_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remainder_bit_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAINDER_BIT_NUM_SPEC;
impl crate::RegisterSpec for REMAINDER_BIT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remainder_bit_num::R`](R) reader structure"]
impl crate::Readable for REMAINDER_BIT_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remainder_bit_num::W`](W) writer structure"]
impl crate::Writable for REMAINDER_BIT_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAINDER_BIT_NUM to value 0"]
impl crate::Resettable for REMAINDER_BIT_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
