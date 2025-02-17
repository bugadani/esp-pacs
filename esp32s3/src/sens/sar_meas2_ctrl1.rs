#[doc = "Register `SAR_MEAS2_CTRL1` reader"]
pub type R = crate::R<SAR_MEAS2_CTRL1_SPEC>;
#[doc = "Register `SAR_MEAS2_CTRL1` writer"]
pub type W = crate::W<SAR_MEAS2_CTRL1_SPEC>;
#[doc = "Field `SAR_SAR2_CNTL_STATE` reader - saradc2_cntl_fsm"]
pub type SAR_SAR2_CNTL_STATE_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_PWDET_CAL_EN` reader - rtc control pwdet enable"]
pub type SAR_SAR2_PWDET_CAL_EN_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_PWDET_CAL_EN` writer - rtc control pwdet enable"]
pub type SAR_SAR2_PWDET_CAL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR_SAR2_PKDET_CAL_EN` reader - rtc control pkdet enable"]
pub type SAR_SAR2_PKDET_CAL_EN_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_PKDET_CAL_EN` writer - rtc control pkdet enable"]
pub type SAR_SAR2_PKDET_CAL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR_SAR2_EN_TEST` reader - SAR2_EN_TEST"]
pub type SAR_SAR2_EN_TEST_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_EN_TEST` writer - SAR2_EN_TEST"]
pub type SAR_SAR2_EN_TEST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR_SAR2_RSTB_FORCE` reader - no public"]
pub type SAR_SAR2_RSTB_FORCE_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_RSTB_FORCE` writer - no public"]
pub type SAR_SAR2_RSTB_FORCE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAR_SAR2_STANDBY_WAIT` reader - no public"]
pub type SAR_SAR2_STANDBY_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_STANDBY_WAIT` writer - no public"]
pub type SAR_SAR2_STANDBY_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SAR_SAR2_RSTB_WAIT` reader - no public"]
pub type SAR_SAR2_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_RSTB_WAIT` writer - no public"]
pub type SAR_SAR2_RSTB_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SAR_SAR2_XPD_WAIT` reader - no public"]
pub type SAR_SAR2_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_XPD_WAIT` writer - no public"]
pub type SAR_SAR2_XPD_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:2 - saradc2_cntl_fsm"]
    #[inline(always)]
    pub fn sar_sar2_cntl_state(&self) -> SAR_SAR2_CNTL_STATE_R {
        SAR_SAR2_CNTL_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - rtc control pwdet enable"]
    #[inline(always)]
    pub fn sar_sar2_pwdet_cal_en(&self) -> SAR_SAR2_PWDET_CAL_EN_R {
        SAR_SAR2_PWDET_CAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rtc control pkdet enable"]
    #[inline(always)]
    pub fn sar_sar2_pkdet_cal_en(&self) -> SAR_SAR2_PKDET_CAL_EN_R {
        SAR_SAR2_PKDET_CAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAR2_EN_TEST"]
    #[inline(always)]
    pub fn sar_sar2_en_test(&self) -> SAR_SAR2_EN_TEST_R {
        SAR_SAR2_EN_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - no public"]
    #[inline(always)]
    pub fn sar_sar2_rstb_force(&self) -> SAR_SAR2_RSTB_FORCE_R {
        SAR_SAR2_RSTB_FORCE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - no public"]
    #[inline(always)]
    pub fn sar_sar2_standby_wait(&self) -> SAR_SAR2_STANDBY_WAIT_R {
        SAR_SAR2_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - no public"]
    #[inline(always)]
    pub fn sar_sar2_rstb_wait(&self) -> SAR_SAR2_RSTB_WAIT_R {
        SAR_SAR2_RSTB_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - no public"]
    #[inline(always)]
    pub fn sar_sar2_xpd_wait(&self) -> SAR_SAR2_XPD_WAIT_R {
        SAR_SAR2_XPD_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS2_CTRL1")
            .field(
                "sar_sar2_cntl_state",
                &format_args!("{}", self.sar_sar2_cntl_state().bits()),
            )
            .field(
                "sar_sar2_pwdet_cal_en",
                &format_args!("{}", self.sar_sar2_pwdet_cal_en().bit()),
            )
            .field(
                "sar_sar2_pkdet_cal_en",
                &format_args!("{}", self.sar_sar2_pkdet_cal_en().bit()),
            )
            .field(
                "sar_sar2_en_test",
                &format_args!("{}", self.sar_sar2_en_test().bit()),
            )
            .field(
                "sar_sar2_rstb_force",
                &format_args!("{}", self.sar_sar2_rstb_force().bits()),
            )
            .field(
                "sar_sar2_standby_wait",
                &format_args!("{}", self.sar_sar2_standby_wait().bits()),
            )
            .field(
                "sar_sar2_rstb_wait",
                &format_args!("{}", self.sar_sar2_rstb_wait().bits()),
            )
            .field(
                "sar_sar2_xpd_wait",
                &format_args!("{}", self.sar_sar2_xpd_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS2_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - rtc control pwdet enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_pwdet_cal_en(&mut self) -> SAR_SAR2_PWDET_CAL_EN_W<SAR_MEAS2_CTRL1_SPEC, 3> {
        SAR_SAR2_PWDET_CAL_EN_W::new(self)
    }
    #[doc = "Bit 4 - rtc control pkdet enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_pkdet_cal_en(&mut self) -> SAR_SAR2_PKDET_CAL_EN_W<SAR_MEAS2_CTRL1_SPEC, 4> {
        SAR_SAR2_PKDET_CAL_EN_W::new(self)
    }
    #[doc = "Bit 5 - SAR2_EN_TEST"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_en_test(&mut self) -> SAR_SAR2_EN_TEST_W<SAR_MEAS2_CTRL1_SPEC, 5> {
        SAR_SAR2_EN_TEST_W::new(self)
    }
    #[doc = "Bits 6:7 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_rstb_force(&mut self) -> SAR_SAR2_RSTB_FORCE_W<SAR_MEAS2_CTRL1_SPEC, 6> {
        SAR_SAR2_RSTB_FORCE_W::new(self)
    }
    #[doc = "Bits 8:15 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_standby_wait(&mut self) -> SAR_SAR2_STANDBY_WAIT_W<SAR_MEAS2_CTRL1_SPEC, 8> {
        SAR_SAR2_STANDBY_WAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_rstb_wait(&mut self) -> SAR_SAR2_RSTB_WAIT_W<SAR_MEAS2_CTRL1_SPEC, 16> {
        SAR_SAR2_RSTB_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_xpd_wait(&mut self) -> SAR_SAR2_XPD_WAIT_W<SAR_MEAS2_CTRL1_SPEC, 24> {
        SAR_SAR2_XPD_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure saradc2 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas2_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas2_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS2_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas2_ctrl1::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS2_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas2_ctrl1::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS2_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS2_CTRL1 to value 0x0702_0200"]
impl crate::Resettable for SAR_MEAS2_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0702_0200;
}
