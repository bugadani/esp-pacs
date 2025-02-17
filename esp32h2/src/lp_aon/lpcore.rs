#[doc = "Register `LPCORE` reader"]
pub type R = crate::R<LPCORE_SPEC>;
#[doc = "Register `LPCORE` writer"]
pub type W = crate::W<LPCORE_SPEC>;
#[doc = "Field `ETM_WAKEUP_FLAG_CLR` writer - need_des"]
pub type ETM_WAKEUP_FLAG_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_WAKEUP_FLAG` reader - need_des"]
pub type ETM_WAKEUP_FLAG_R = crate::BitReader;
#[doc = "Field `ETM_WAKEUP_FLAG` writer - need_des"]
pub type ETM_WAKEUP_FLAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISABLE` reader - need_des"]
pub type DISABLE_R = crate::BitReader;
#[doc = "Field `DISABLE` writer - need_des"]
pub type DISABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn etm_wakeup_flag(&self) -> ETM_WAKEUP_FLAG_R {
        ETM_WAKEUP_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCORE")
            .field(
                "etm_wakeup_flag",
                &format_args!("{}", self.etm_wakeup_flag().bit()),
            )
            .field("disable", &format_args!("{}", self.disable().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPCORE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn etm_wakeup_flag_clr(&mut self) -> ETM_WAKEUP_FLAG_CLR_W<LPCORE_SPEC, 0> {
        ETM_WAKEUP_FLAG_CLR_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn etm_wakeup_flag(&mut self) -> ETM_WAKEUP_FLAG_W<LPCORE_SPEC, 1> {
        ETM_WAKEUP_FLAG_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DISABLE_W<LPCORE_SPEC, 31> {
        DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcore::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcore::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCORE_SPEC;
impl crate::RegisterSpec for LPCORE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcore::R`](R) reader structure"]
impl crate::Readable for LPCORE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpcore::W`](W) writer structure"]
impl crate::Writable for LPCORE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPCORE to value 0"]
impl crate::Resettable for LPCORE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
