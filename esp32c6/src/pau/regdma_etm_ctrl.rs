#[doc = "Register `REGDMA_ETM_CTRL` writer"]
pub type W = crate::W<REGDMA_ETM_CTRL_SPEC>;
#[doc = "Field `ETM_START_0` writer - etm_start_0 reg"]
pub type ETM_START_0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_START_1` writer - etm_start_1 reg"]
pub type ETM_START_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_START_2` writer - etm_start_2 reg"]
pub type ETM_START_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_START_3` writer - etm_start_3 reg"]
pub type ETM_START_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_ETM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - etm_start_0 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_0(&mut self) -> ETM_START_0_W<REGDMA_ETM_CTRL_SPEC, 0> {
        ETM_START_0_W::new(self)
    }
    #[doc = "Bit 1 - etm_start_1 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_1(&mut self) -> ETM_START_1_W<REGDMA_ETM_CTRL_SPEC, 1> {
        ETM_START_1_W::new(self)
    }
    #[doc = "Bit 2 - etm_start_2 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_2(&mut self) -> ETM_START_2_W<REGDMA_ETM_CTRL_SPEC, 2> {
        ETM_START_2_W::new(self)
    }
    #[doc = "Bit 3 - etm_start_3 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_3(&mut self) -> ETM_START_3_W<REGDMA_ETM_CTRL_SPEC, 3> {
        ETM_START_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ETM start ctrl reg\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_etm_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_ETM_CTRL_SPEC;
impl crate::RegisterSpec for REGDMA_ETM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`regdma_etm_ctrl::W`](W) writer structure"]
impl crate::Writable for REGDMA_ETM_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_ETM_CTRL to value 0"]
impl crate::Resettable for REGDMA_ETM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
