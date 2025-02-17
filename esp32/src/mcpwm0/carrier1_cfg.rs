#[doc = "Register `CARRIER1_CFG` reader"]
pub type R = crate::R<CARRIER1_CFG_SPEC>;
#[doc = "Register `CARRIER1_CFG` writer"]
pub type W = crate::W<CARRIER1_CFG_SPEC>;
#[doc = "Field `CARRIER1_EN` reader - "]
pub type CARRIER1_EN_R = crate::BitReader;
#[doc = "Field `CARRIER1_EN` writer - "]
pub type CARRIER1_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARRIER1_PRESCALE` reader - "]
pub type CARRIER1_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CARRIER1_PRESCALE` writer - "]
pub type CARRIER1_PRESCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CARRIER1_DUTY` reader - "]
pub type CARRIER1_DUTY_R = crate::FieldReader;
#[doc = "Field `CARRIER1_DUTY` writer - "]
pub type CARRIER1_DUTY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CARRIER1_OSHTWTH` reader - "]
pub type CARRIER1_OSHTWTH_R = crate::FieldReader;
#[doc = "Field `CARRIER1_OSHTWTH` writer - "]
pub type CARRIER1_OSHTWTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CARRIER1_OUT_INVERT` reader - "]
pub type CARRIER1_OUT_INVERT_R = crate::BitReader;
#[doc = "Field `CARRIER1_OUT_INVERT` writer - "]
pub type CARRIER1_OUT_INVERT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARRIER1_IN_INVERT` reader - "]
pub type CARRIER1_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CARRIER1_IN_INVERT` writer - "]
pub type CARRIER1_IN_INVERT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn carrier1_en(&self) -> CARRIER1_EN_R {
        CARRIER1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn carrier1_prescale(&self) -> CARRIER1_PRESCALE_R {
        CARRIER1_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn carrier1_duty(&self) -> CARRIER1_DUTY_R {
        CARRIER1_DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn carrier1_oshtwth(&self) -> CARRIER1_OSHTWTH_R {
        CARRIER1_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn carrier1_out_invert(&self) -> CARRIER1_OUT_INVERT_R {
        CARRIER1_OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn carrier1_in_invert(&self) -> CARRIER1_IN_INVERT_R {
        CARRIER1_IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER1_CFG")
            .field("carrier1_en", &format_args!("{}", self.carrier1_en().bit()))
            .field(
                "carrier1_prescale",
                &format_args!("{}", self.carrier1_prescale().bits()),
            )
            .field(
                "carrier1_duty",
                &format_args!("{}", self.carrier1_duty().bits()),
            )
            .field(
                "carrier1_oshtwth",
                &format_args!("{}", self.carrier1_oshtwth().bits()),
            )
            .field(
                "carrier1_out_invert",
                &format_args!("{}", self.carrier1_out_invert().bit()),
            )
            .field(
                "carrier1_in_invert",
                &format_args!("{}", self.carrier1_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CARRIER1_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn carrier1_en(&mut self) -> CARRIER1_EN_W<CARRIER1_CFG_SPEC, 0> {
        CARRIER1_EN_W::new(self)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    #[must_use]
    pub fn carrier1_prescale(&mut self) -> CARRIER1_PRESCALE_W<CARRIER1_CFG_SPEC, 1> {
        CARRIER1_PRESCALE_W::new(self)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn carrier1_duty(&mut self) -> CARRIER1_DUTY_W<CARRIER1_CFG_SPEC, 5> {
        CARRIER1_DUTY_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn carrier1_oshtwth(&mut self) -> CARRIER1_OSHTWTH_W<CARRIER1_CFG_SPEC, 8> {
        CARRIER1_OSHTWTH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn carrier1_out_invert(&mut self) -> CARRIER1_OUT_INVERT_W<CARRIER1_CFG_SPEC, 12> {
        CARRIER1_OUT_INVERT_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn carrier1_in_invert(&mut self) -> CARRIER1_IN_INVERT_W<CARRIER1_CFG_SPEC, 13> {
        CARRIER1_IN_INVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier1_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier1_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER1_CFG_SPEC;
impl crate::RegisterSpec for CARRIER1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier1_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER1_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier1_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER1_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CARRIER1_CFG to value 0"]
impl crate::Resettable for CARRIER1_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
