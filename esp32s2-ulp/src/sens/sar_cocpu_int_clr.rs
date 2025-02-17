#[doc = "Register `SAR_COCPU_INT_CLR` writer"]
pub type W = crate::W<SAR_COCPU_INT_CLR_SPEC>;
#[doc = "Field `COCPU_TOUCH_DONE_INT_CLR` writer - TOUCH_DONE_INT interrupt clear bit"]
pub type COCPU_TOUCH_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_CLR` writer - TOUCH_INACTIVE_INT interrupt clear bit"]
pub type COCPU_TOUCH_INACTIVE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_CLR` writer - TOUCH_ACTIVE_INT interrupt clear bit"]
pub type COCPU_TOUCH_ACTIVE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_SARADC1_INT_CLR` writer - SARADC1_DONE_INT interrupt clear bit"]
pub type COCPU_SARADC1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_SARADC2_INT_CLR` writer - SARADC2_DONE_INT interrupt clear bit"]
pub type COCPU_SARADC2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_TSENS_INT_CLR` writer - TSENS_DONE_INT interrupt clear bit"]
pub type COCPU_TSENS_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_START_INT_CLR` writer - RISCV_START_INT interrupt clear bit"]
pub type COCPU_START_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_SW_INT_CLR` writer - SW_INT interrupt clear bit"]
pub type COCPU_SW_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COCPU_SWD_INT_CLR` writer - SWD_INT interrupt clear bit"]
pub type COCPU_SWD_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_done_int_clr(
        &mut self,
    ) -> COCPU_TOUCH_DONE_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 0> {
        COCPU_TOUCH_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_inactive_int_clr(
        &mut self,
    ) -> COCPU_TOUCH_INACTIVE_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 1> {
        COCPU_TOUCH_INACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_active_int_clr(
        &mut self,
    ) -> COCPU_TOUCH_ACTIVE_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 2> {
        COCPU_TOUCH_ACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc1_int_clr(&mut self) -> COCPU_SARADC1_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 3> {
        COCPU_SARADC1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc2_int_clr(&mut self) -> COCPU_SARADC2_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 4> {
        COCPU_SARADC2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_tsens_int_clr(&mut self) -> COCPU_TSENS_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 5> {
        COCPU_TSENS_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_start_int_clr(&mut self) -> COCPU_START_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 6> {
        COCPU_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - SW_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_sw_int_clr(&mut self) -> COCPU_SW_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 7> {
        COCPU_SW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - SWD_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_swd_int_clr(&mut self) -> COCPU_SWD_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC, 8> {
        COCPU_SWD_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear bit of ULP-RISCV\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_INT_CLR_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sar_cocpu_int_clr::W`](W) writer structure"]
impl crate::Writable for SAR_COCPU_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_CLR to value 0"]
impl crate::Resettable for SAR_COCPU_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
