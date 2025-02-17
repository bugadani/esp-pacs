#[doc = "Register `LOG_MAX` reader"]
pub type R = crate::R<LOG_MAX_SPEC>;
#[doc = "Register `LOG_MAX` writer"]
pub type W = crate::W<LOG_MAX_SPEC>;
#[doc = "Field `LOG_MAX` reader - the max address of log range"]
pub type LOG_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `LOG_MAX` writer - the max address of log range"]
pub type LOG_MAX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - the max address of log range"]
    #[inline(always)]
    pub fn log_max(&self) -> LOG_MAX_R {
        LOG_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MAX")
            .field("log_max", &format_args!("{}", self.log_max().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_MAX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the max address of log range"]
    #[inline(always)]
    #[must_use]
    pub fn log_max(&mut self) -> LOG_MAX_W<LOG_MAX_SPEC, 0> {
        LOG_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "log boundary regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_max::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_max::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_MAX_SPEC;
impl crate::RegisterSpec for LOG_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_max::R`](R) reader structure"]
impl crate::Readable for LOG_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_max::W`](W) writer structure"]
impl crate::Writable for LOG_MAX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_MAX to value 0"]
impl crate::Resettable for LOG_MAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
