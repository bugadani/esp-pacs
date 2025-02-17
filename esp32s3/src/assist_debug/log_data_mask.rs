#[doc = "Register `LOG_DATA_MASK` reader"]
pub type R = crate::R<LOG_DATA_MASK_SPEC>;
#[doc = "Register `LOG_DATA_MASK` writer"]
pub type W = crate::W<LOG_DATA_MASK_SPEC>;
#[doc = "Field `LOG_DATA_SIZE` reader - data mask"]
pub type LOG_DATA_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `LOG_DATA_SIZE` writer - data mask"]
pub type LOG_DATA_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - data mask"]
    #[inline(always)]
    pub fn log_data_size(&self) -> LOG_DATA_SIZE_R {
        LOG_DATA_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_MASK")
            .field(
                "log_data_size",
                &format_args!("{}", self.log_data_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - data mask"]
    #[inline(always)]
    #[must_use]
    pub fn log_data_size(&mut self) -> LOG_DATA_SIZE_W<LOG_DATA_MASK_SPEC, 0> {
        LOG_DATA_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "log check data mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_DATA_MASK_SPEC;
impl crate::RegisterSpec for LOG_DATA_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_data_mask::R`](R) reader structure"]
impl crate::Readable for LOG_DATA_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_data_mask::W`](W) writer structure"]
impl crate::Writable for LOG_DATA_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_DATA_MASK to value 0"]
impl crate::Resettable for LOG_DATA_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
