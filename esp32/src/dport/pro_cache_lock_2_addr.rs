#[doc = "Register `PRO_CACHE_LOCK_2_ADDR` reader"]
pub type R = crate::R<PRO_CACHE_LOCK_2_ADDR_SPEC>;
#[doc = "Register `PRO_CACHE_LOCK_2_ADDR` writer"]
pub type W = crate::W<PRO_CACHE_LOCK_2_ADDR_SPEC>;
#[doc = "Field `PRE` reader - "]
pub type PRE_R = crate::FieldReader<u16>;
#[doc = "Field `PRE` writer - "]
pub type PRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `MIN` reader - "]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `MIN` writer - "]
pub type MIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MAX` reader - "]
pub type MAX_R = crate::FieldReader;
#[doc = "Field `MAX` writer - "]
pub type MAX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_LOCK_2_ADDR")
            .field("pre", &format_args!("{}", self.pre().bits()))
            .field("min", &format_args!("{}", self.min().bits()))
            .field("max", &format_args!("{}", self.max().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_LOCK_2_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<PRO_CACHE_LOCK_2_ADDR_SPEC, 0> {
        PRE_W::new(self)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<PRO_CACHE_LOCK_2_ADDR_SPEC, 14> {
        MIN_W::new(self)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    #[must_use]
    pub fn max(&mut self) -> MAX_W<PRO_CACHE_LOCK_2_ADDR_SPEC, 18> {
        MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_lock_2_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_lock_2_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_LOCK_2_ADDR_SPEC;
impl crate::RegisterSpec for PRO_CACHE_LOCK_2_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_lock_2_addr::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_LOCK_2_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_lock_2_addr::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_LOCK_2_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_LOCK_2_ADDR to value 0"]
impl crate::Resettable for PRO_CACHE_LOCK_2_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
