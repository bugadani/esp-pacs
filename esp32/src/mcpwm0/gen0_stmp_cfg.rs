#[doc = "Register `GEN0_STMP_CFG` reader"]
pub struct R(crate::R<GEN0_STMP_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN0_STMP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN0_STMP_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN0_STMP_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN0_STMP_CFG` writer"]
pub struct W(crate::W<GEN0_STMP_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN0_STMP_CFG_SPEC>;
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
impl From<crate::W<GEN0_STMP_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN0_STMP_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN0_A_UPMETHOD` reader - "]
pub type GEN0_A_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN0_A_UPMETHOD` writer - "]
pub type GEN0_A_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_STMP_CFG_SPEC, 4, O>;
#[doc = "Field `GEN0_B_UPMETHOD` reader - "]
pub type GEN0_B_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN0_B_UPMETHOD` writer - "]
pub type GEN0_B_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_STMP_CFG_SPEC, 4, O>;
#[doc = "Field `GEN0_A_SHDW_FULL` reader - "]
pub type GEN0_A_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `GEN0_A_SHDW_FULL` writer - "]
pub type GEN0_A_SHDW_FULL_W<'a, const O: u8> = crate::BitWriter<'a, GEN0_STMP_CFG_SPEC, O>;
#[doc = "Field `GEN0_B_SHDW_FULL` reader - "]
pub type GEN0_B_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `GEN0_B_SHDW_FULL` writer - "]
pub type GEN0_B_SHDW_FULL_W<'a, const O: u8> = crate::BitWriter<'a, GEN0_STMP_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen0_a_upmethod(&self) -> GEN0_A_UPMETHOD_R {
        GEN0_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn gen0_b_upmethod(&self) -> GEN0_B_UPMETHOD_R {
        GEN0_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gen0_a_shdw_full(&self) -> GEN0_A_SHDW_FULL_R {
        GEN0_A_SHDW_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gen0_b_shdw_full(&self) -> GEN0_B_SHDW_FULL_R {
        GEN0_B_SHDW_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN0_STMP_CFG")
            .field(
                "gen0_a_upmethod",
                &format_args!("{}", self.gen0_a_upmethod().bits()),
            )
            .field(
                "gen0_b_upmethod",
                &format_args!("{}", self.gen0_b_upmethod().bits()),
            )
            .field(
                "gen0_a_shdw_full",
                &format_args!("{}", self.gen0_a_shdw_full().bit()),
            )
            .field(
                "gen0_b_shdw_full",
                &format_args!("{}", self.gen0_b_shdw_full().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN0_STMP_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_a_upmethod(&mut self) -> GEN0_A_UPMETHOD_W<0> {
        GEN0_A_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_b_upmethod(&mut self) -> GEN0_B_UPMETHOD_W<4> {
        GEN0_B_UPMETHOD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_a_shdw_full(&mut self) -> GEN0_A_SHDW_FULL_W<8> {
        GEN0_A_SHDW_FULL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_b_shdw_full(&mut self) -> GEN0_B_SHDW_FULL_W<9> {
        GEN0_B_SHDW_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen0_stmp_cfg](index.html) module"]
pub struct GEN0_STMP_CFG_SPEC;
impl crate::RegisterSpec for GEN0_STMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen0_stmp_cfg::R](R) reader structure"]
impl crate::Readable for GEN0_STMP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen0_stmp_cfg::W](W) writer structure"]
impl crate::Writable for GEN0_STMP_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN0_STMP_CFG to value 0"]
impl crate::Resettable for GEN0_STMP_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
