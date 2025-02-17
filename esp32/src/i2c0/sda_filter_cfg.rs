#[doc = "Register `SDA_FILTER_CFG` reader"]
pub type R = crate::R<SDA_FILTER_CFG_SPEC>;
#[doc = "Register `SDA_FILTER_CFG` writer"]
pub type W = crate::W<SDA_FILTER_CFG_SPEC>;
#[doc = "Field `SDA_FILTER_THRES` reader - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub type SDA_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `SDA_FILTER_THRES` writer - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub type SDA_FILTER_THRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SDA_FILTER_EN` reader - This is the filter enable bit for SDA."]
pub type SDA_FILTER_EN_R = crate::BitReader;
#[doc = "Field `SDA_FILTER_EN` writer - This is the filter enable bit for SDA."]
pub type SDA_FILTER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> SDA_FILTER_THRES_R {
        SDA_FILTER_THRES_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn sda_filter_en(&self) -> SDA_FILTER_EN_R {
        SDA_FILTER_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_FILTER_CFG")
            .field(
                "sda_filter_thres",
                &format_args!("{}", self.sda_filter_thres().bits()),
            )
            .field(
                "sda_filter_en",
                &format_args!("{}", self.sda_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_FILTER_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    #[must_use]
    pub fn sda_filter_thres(&mut self) -> SDA_FILTER_THRES_W<SDA_FILTER_CFG_SPEC, 0> {
        SDA_FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 3 - This is the filter enable bit for SDA."]
    #[inline(always)]
    #[must_use]
    pub fn sda_filter_en(&mut self) -> SDA_FILTER_EN_W<SDA_FILTER_CFG_SPEC, 3> {
        SDA_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_filter_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_filter_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_FILTER_CFG_SPEC;
impl crate::RegisterSpec for SDA_FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_filter_cfg::R`](R) reader structure"]
impl crate::Readable for SDA_FILTER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_filter_cfg::W`](W) writer structure"]
impl crate::Writable for SDA_FILTER_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDA_FILTER_CFG to value 0x08"]
impl crate::Resettable for SDA_FILTER_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
