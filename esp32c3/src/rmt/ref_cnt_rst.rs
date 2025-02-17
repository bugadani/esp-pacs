#[doc = "Register `REF_CNT_RST` writer"]
pub type W = crate::W<REF_CNT_RST_SPEC>;
#[doc = "Field `CH0` writer - reg_ref_cnt_rst_ch0."]
pub type CH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1` writer - reg_ref_cnt_rst_ch1."]
pub type CH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2` writer - reg_ref_cnt_rst_ch2."]
pub type CH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3` writer - reg_ref_cnt_rst_ch3."]
pub type CH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CNT_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reg_ref_cnt_rst_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<REF_CNT_RST_SPEC, 0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - reg_ref_cnt_rst_ch1."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<REF_CNT_RST_SPEC, 1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - reg_ref_cnt_rst_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<REF_CNT_RST_SPEC, 2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - reg_ref_cnt_rst_ch3."]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<REF_CNT_RST_SPEC, 3> {
        CH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RMT_REF_CNT_RST_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ref_cnt_rst::W`](W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
