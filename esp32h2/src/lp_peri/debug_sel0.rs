#[doc = "Register `DEBUG_SEL0` reader"]
pub type R = crate::R<DEBUG_SEL0_SPEC>;
#[doc = "Register `DEBUG_SEL0` writer"]
pub type W = crate::W<DEBUG_SEL0_SPEC>;
#[doc = "Field `DEBUG_SEL0` reader - need des"]
pub type DEBUG_SEL0_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL0` writer - need des"]
pub type DEBUG_SEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DEBUG_SEL1` reader - need des"]
pub type DEBUG_SEL1_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL1` writer - need des"]
pub type DEBUG_SEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DEBUG_SEL2` reader - need des"]
pub type DEBUG_SEL2_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL2` writer - need des"]
pub type DEBUG_SEL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DEBUG_SEL3` reader - need des"]
pub type DEBUG_SEL3_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL3` writer - need des"]
pub type DEBUG_SEL3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    pub fn debug_sel0(&self) -> DEBUG_SEL0_R {
        DEBUG_SEL0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - need des"]
    #[inline(always)]
    pub fn debug_sel1(&self) -> DEBUG_SEL1_R {
        DEBUG_SEL1_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - need des"]
    #[inline(always)]
    pub fn debug_sel2(&self) -> DEBUG_SEL2_R {
        DEBUG_SEL2_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - need des"]
    #[inline(always)]
    pub fn debug_sel3(&self) -> DEBUG_SEL3_R {
        DEBUG_SEL3_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL0")
            .field("debug_sel0", &format_args!("{}", self.debug_sel0().bits()))
            .field("debug_sel1", &format_args!("{}", self.debug_sel1().bits()))
            .field("debug_sel2", &format_args!("{}", self.debug_sel2().bits()))
            .field("debug_sel3", &format_args!("{}", self.debug_sel3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEBUG_SEL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel0(&mut self) -> DEBUG_SEL0_W<DEBUG_SEL0_SPEC, 0> {
        DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 7:13 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel1(&mut self) -> DEBUG_SEL1_W<DEBUG_SEL0_SPEC, 7> {
        DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 14:20 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel2(&mut self) -> DEBUG_SEL2_W<DEBUG_SEL0_SPEC, 14> {
        DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 21:27 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel3(&mut self) -> DEBUG_SEL3_W<DEBUG_SEL0_SPEC, 21> {
        DEBUG_SEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_sel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SEL0_SPEC;
impl crate::RegisterSpec for DEBUG_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_sel0::R`](R) reader structure"]
impl crate::Readable for DEBUG_SEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_sel0::W`](W) writer structure"]
impl crate::Writable for DEBUG_SEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL0 to value 0"]
impl crate::Resettable for DEBUG_SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
