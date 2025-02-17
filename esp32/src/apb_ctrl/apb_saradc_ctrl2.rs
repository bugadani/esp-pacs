#[doc = "Register `APB_SARADC_CTRL2` reader"]
pub type R = crate::R<APB_SARADC_CTRL2_SPEC>;
#[doc = "Register `APB_SARADC_CTRL2` writer"]
pub type W = crate::W<APB_SARADC_CTRL2_SPEC>;
#[doc = "Field `SARADC_MEAS_NUM_LIMIT` reader - "]
pub type SARADC_MEAS_NUM_LIMIT_R = crate::BitReader;
#[doc = "Field `SARADC_MEAS_NUM_LIMIT` writer - "]
pub type SARADC_MEAS_NUM_LIMIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARADC_MAX_MEAS_NUM` reader - max conversion number"]
pub type SARADC_MAX_MEAS_NUM_R = crate::FieldReader;
#[doc = "Field `SARADC_MAX_MEAS_NUM` writer - max conversion number"]
pub type SARADC_MAX_MEAS_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SARADC_SAR1_INV` reader - 1: data to DIG ADC1 CTRL is inverted otherwise not"]
pub type SARADC_SAR1_INV_R = crate::BitReader;
#[doc = "Field `SARADC_SAR1_INV` writer - 1: data to DIG ADC1 CTRL is inverted otherwise not"]
pub type SARADC_SAR1_INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARADC_SAR2_INV` reader - 1: data to DIG ADC2 CTRL is inverted otherwise not"]
pub type SARADC_SAR2_INV_R = crate::BitReader;
#[doc = "Field `SARADC_SAR2_INV` writer - 1: data to DIG ADC2 CTRL is inverted otherwise not"]
pub type SARADC_SAR2_INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn saradc_meas_num_limit(&self) -> SARADC_MEAS_NUM_LIMIT_R {
        SARADC_MEAS_NUM_LIMIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    pub fn saradc_max_meas_num(&self) -> SARADC_MAX_MEAS_NUM_R {
        SARADC_MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted otherwise not"]
    #[inline(always)]
    pub fn saradc_sar1_inv(&self) -> SARADC_SAR1_INV_R {
        SARADC_SAR1_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted otherwise not"]
    #[inline(always)]
    pub fn saradc_sar2_inv(&self) -> SARADC_SAR2_INV_R {
        SARADC_SAR2_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC_CTRL2")
            .field(
                "saradc_meas_num_limit",
                &format_args!("{}", self.saradc_meas_num_limit().bit()),
            )
            .field(
                "saradc_max_meas_num",
                &format_args!("{}", self.saradc_max_meas_num().bits()),
            )
            .field(
                "saradc_sar1_inv",
                &format_args!("{}", self.saradc_sar1_inv().bit()),
            )
            .field(
                "saradc_sar2_inv",
                &format_args!("{}", self.saradc_sar2_inv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_SARADC_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_meas_num_limit(&mut self) -> SARADC_MEAS_NUM_LIMIT_W<APB_SARADC_CTRL2_SPEC, 0> {
        SARADC_MEAS_NUM_LIMIT_W::new(self)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_max_meas_num(&mut self) -> SARADC_MAX_MEAS_NUM_W<APB_SARADC_CTRL2_SPEC, 1> {
        SARADC_MAX_MEAS_NUM_W::new(self)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted otherwise not"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_inv(&mut self) -> SARADC_SAR1_INV_W<APB_SARADC_CTRL2_SPEC, 9> {
        SARADC_SAR1_INV_W::new(self)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted otherwise not"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar2_inv(&mut self) -> SARADC_SAR2_INV_W<APB_SARADC_CTRL2_SPEC, 10> {
        SARADC_SAR2_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_SARADC_CTRL2_SPEC;
impl crate::RegisterSpec for APB_SARADC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_saradc_ctrl2::R`](R) reader structure"]
impl crate::Readable for APB_SARADC_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_saradc_ctrl2::W`](W) writer structure"]
impl crate::Writable for APB_SARADC_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_SARADC_CTRL2 to value 0x01fe"]
impl crate::Resettable for APB_SARADC_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01fe;
}
