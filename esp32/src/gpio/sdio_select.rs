#[doc = "Register `SDIO_SELECT` reader"]
pub type R = crate::R<SDIO_SELECT_SPEC>;
#[doc = "Register `SDIO_SELECT` writer"]
pub type W = crate::W<SDIO_SELECT_SPEC>;
#[doc = "Field `SDIO_SEL` reader - SDIO PADS on/off control from outside"]
pub type SDIO_SEL_R = crate::FieldReader;
#[doc = "Field `SDIO_SEL` writer - SDIO PADS on/off control from outside"]
pub type SDIO_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SDIO PADS on/off control from outside"]
    #[inline(always)]
    pub fn sdio_sel(&self) -> SDIO_SEL_R {
        SDIO_SEL_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SELECT")
            .field("sdio_sel", &format_args!("{}", self.sdio_sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_SELECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDIO PADS on/off control from outside"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_sel(&mut self) -> SDIO_SEL_W<SDIO_SELECT_SPEC, 0> {
        SDIO_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_SELECT_SPEC;
impl crate::RegisterSpec for SDIO_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_select::R`](R) reader structure"]
impl crate::Readable for SDIO_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_select::W`](W) writer structure"]
impl crate::Writable for SDIO_SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_SELECT to value 0"]
impl crate::Resettable for SDIO_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
