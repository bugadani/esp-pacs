#[doc = "Register `PRO_CACHE_2` reader"]
pub struct R(crate::R<PRO_CACHE_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_2` writer"]
pub struct W(crate::W<PRO_CACHE_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRO_CACHE_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_ILG_CLR` reader - The clear signal for cache access interrupt."]
pub type PRO_CACHE_ILG_CLR_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_ILG_CLR` writer - The clear signal for cache access interrupt."]
pub type PRO_CACHE_ILG_CLR_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_2_SPEC, O>;
#[doc = "Field `PRO_CACHE_ILG_EN` reader - The enable signal for cache access interrupt."]
pub type PRO_CACHE_ILG_EN_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_ILG_EN` writer - The enable signal for cache access interrupt."]
pub type PRO_CACHE_ILG_EN_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_2_SPEC, O>;
#[doc = "Field `PRO_CACHE_ILG_INTR` reader - Cache access interrupt signal."]
pub type PRO_CACHE_ILG_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for cache access interrupt."]
    #[inline(always)]
    pub fn pro_cache_ilg_clr(&self) -> PRO_CACHE_ILG_CLR_R {
        PRO_CACHE_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for cache access interrupt."]
    #[inline(always)]
    pub fn pro_cache_ilg_en(&self) -> PRO_CACHE_ILG_EN_R {
        PRO_CACHE_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache access interrupt signal."]
    #[inline(always)]
    pub fn pro_cache_ilg_intr(&self) -> PRO_CACHE_ILG_INTR_R {
        PRO_CACHE_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_2")
            .field(
                "pro_cache_ilg_clr",
                &format_args!("{}", self.pro_cache_ilg_clr().bit()),
            )
            .field(
                "pro_cache_ilg_en",
                &format_args!("{}", self.pro_cache_ilg_en().bit()),
            )
            .field(
                "pro_cache_ilg_intr",
                &format_args!("{}", self.pro_cache_ilg_intr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for cache access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_ilg_clr(&mut self) -> PRO_CACHE_ILG_CLR_W<0> {
        PRO_CACHE_ILG_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The enable signal for cache access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_ilg_en(&mut self) -> PRO_CACHE_ILG_EN_W<1> {
        PRO_CACHE_ILG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache permission control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_2](index.html) module"]
pub struct PRO_CACHE_2_SPEC;
impl crate::RegisterSpec for PRO_CACHE_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_2::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_2::W](W) writer structure"]
impl crate::Writable for PRO_CACHE_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_2 to value 0"]
impl crate::Resettable for PRO_CACHE_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
