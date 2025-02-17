#[doc = "Register `DIG_PAD_HOLD` reader"]
pub type R = crate::R<DIG_PAD_HOLD_SPEC>;
#[doc = "Register `DIG_PAD_HOLD` writer"]
pub type W = crate::W<DIG_PAD_HOLD_SPEC>;
#[doc = "Field `DIG_PAD_HOLD` reader - configure digtal pad hold"]
pub type DIG_PAD_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `DIG_PAD_HOLD` writer - configure digtal pad hold"]
pub type DIG_PAD_HOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - configure digtal pad hold"]
    #[inline(always)]
    pub fn dig_pad_hold(&self) -> DIG_PAD_HOLD_R {
        DIG_PAD_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_PAD_HOLD")
            .field(
                "dig_pad_hold",
                &format_args!("{}", self.dig_pad_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIG_PAD_HOLD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - configure digtal pad hold"]
    #[inline(always)]
    #[must_use]
    pub fn dig_pad_hold(&mut self) -> DIG_PAD_HOLD_W<DIG_PAD_HOLD_SPEC, 0> {
        DIG_PAD_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure digtal pad hold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pad_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pad_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIG_PAD_HOLD_SPEC;
impl crate::RegisterSpec for DIG_PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_pad_hold::R`](R) reader structure"]
impl crate::Readable for DIG_PAD_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dig_pad_hold::W`](W) writer structure"]
impl crate::Writable for DIG_PAD_HOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIG_PAD_HOLD to value 0"]
impl crate::Resettable for DIG_PAD_HOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
