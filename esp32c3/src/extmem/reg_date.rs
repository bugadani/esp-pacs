#[doc = "Register `REG_DATE` reader"]
pub type R = crate::R<REG_DATE_SPEC>;
#[doc = "Register `REG_DATE` writer"]
pub type W = crate::W<REG_DATE_SPEC>;
#[doc = "Field `DATE` reader - version information"]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - version information"]
pub type DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - version information"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_DATE")
            .field("date", &format_args!("{}", self.date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - version information"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<REG_DATE_SPEC, 0> {
        DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_DATE_SPEC;
impl crate::RegisterSpec for REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_date::R`](R) reader structure"]
impl crate::Readable for REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_date::W`](W) writer structure"]
impl crate::Writable for REG_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_DATE to value 0x0200_7160"]
impl crate::Resettable for REG_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_7160;
}
