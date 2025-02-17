#[doc = "Register `PLC_CONF2` reader"]
pub type R = crate::R<PLC_CONF2_SPEC>;
#[doc = "Register `PLC_CONF2` writer"]
pub type W = crate::W<PLC_CONF2_SPEC>;
#[doc = "Field `CVSD_SEG_MOD` reader - "]
pub type CVSD_SEG_MOD_R = crate::FieldReader;
#[doc = "Field `CVSD_SEG_MOD` writer - "]
pub type CVSD_SEG_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MIN_PERIOD` reader - "]
pub type MIN_PERIOD_R = crate::FieldReader;
#[doc = "Field `MIN_PERIOD` writer - "]
pub type MIN_PERIOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cvsd_seg_mod(&self) -> CVSD_SEG_MOD_R {
        CVSD_SEG_MOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn min_period(&self) -> MIN_PERIOD_R {
        MIN_PERIOD_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLC_CONF2")
            .field(
                "cvsd_seg_mod",
                &format_args!("{}", self.cvsd_seg_mod().bits()),
            )
            .field("min_period", &format_args!("{}", self.min_period().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLC_CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_seg_mod(&mut self) -> CVSD_SEG_MOD_W<PLC_CONF2_SPEC, 0> {
        CVSD_SEG_MOD_W::new(self)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    #[must_use]
    pub fn min_period(&mut self) -> MIN_PERIOD_W<PLC_CONF2_SPEC, 2> {
        MIN_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLC_CONF2_SPEC;
impl crate::RegisterSpec for PLC_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plc_conf2::R`](R) reader structure"]
impl crate::Readable for PLC_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plc_conf2::W`](W) writer structure"]
impl crate::Writable for PLC_CONF2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLC_CONF2 to value 0x28"]
impl crate::Resettable for PLC_CONF2_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
