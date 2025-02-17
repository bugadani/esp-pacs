#[doc = "Register `PIN7` reader"]
pub type R = crate::R<PIN7_SPEC>;
#[doc = "Register `PIN7` writer"]
pub type W = crate::W<PIN7_SPEC>;
#[doc = "Field `LP_GPIO7_SYNC_BYPASS` reader - need des"]
pub type LP_GPIO7_SYNC_BYPASS_R = crate::FieldReader;
#[doc = "Field `LP_GPIO7_SYNC_BYPASS` writer - need des"]
pub type LP_GPIO7_SYNC_BYPASS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LP_GPIO7_PAD_DRIVER` reader - need des"]
pub type LP_GPIO7_PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `LP_GPIO7_PAD_DRIVER` writer - need des"]
pub type LP_GPIO7_PAD_DRIVER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_GPIO7_EDGE_WAKEUP_CLR` writer - need des"]
pub type LP_GPIO7_EDGE_WAKEUP_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_GPIO7_INT_TYPE` reader - need des"]
pub type LP_GPIO7_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `LP_GPIO7_INT_TYPE` writer - need des"]
pub type LP_GPIO7_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LP_GPIO7_WAKEUP_ENABLE` reader - need des"]
pub type LP_GPIO7_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `LP_GPIO7_WAKEUP_ENABLE` writer - need des"]
pub type LP_GPIO7_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_GPIO7_FILTER_EN` reader - need des"]
pub type LP_GPIO7_FILTER_EN_R = crate::BitReader;
#[doc = "Field `LP_GPIO7_FILTER_EN` writer - need des"]
pub type LP_GPIO7_FILTER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    pub fn lp_gpio7_sync_bypass(&self) -> LP_GPIO7_SYNC_BYPASS_R {
        LP_GPIO7_SYNC_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn lp_gpio7_pad_driver(&self) -> LP_GPIO7_PAD_DRIVER_R {
        LP_GPIO7_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    pub fn lp_gpio7_int_type(&self) -> LP_GPIO7_INT_TYPE_R {
        LP_GPIO7_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    pub fn lp_gpio7_wakeup_enable(&self) -> LP_GPIO7_WAKEUP_ENABLE_R {
        LP_GPIO7_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn lp_gpio7_filter_en(&self) -> LP_GPIO7_FILTER_EN_R {
        LP_GPIO7_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN7")
            .field(
                "lp_gpio7_sync_bypass",
                &format_args!("{}", self.lp_gpio7_sync_bypass().bits()),
            )
            .field(
                "lp_gpio7_pad_driver",
                &format_args!("{}", self.lp_gpio7_pad_driver().bit()),
            )
            .field(
                "lp_gpio7_int_type",
                &format_args!("{}", self.lp_gpio7_int_type().bits()),
            )
            .field(
                "lp_gpio7_wakeup_enable",
                &format_args!("{}", self.lp_gpio7_wakeup_enable().bit()),
            )
            .field(
                "lp_gpio7_filter_en",
                &format_args!("{}", self.lp_gpio7_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio7_sync_bypass(&mut self) -> LP_GPIO7_SYNC_BYPASS_W<PIN7_SPEC, 0> {
        LP_GPIO7_SYNC_BYPASS_W::new(self)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio7_pad_driver(&mut self) -> LP_GPIO7_PAD_DRIVER_W<PIN7_SPEC, 2> {
        LP_GPIO7_PAD_DRIVER_W::new(self)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio7_edge_wakeup_clr(&mut self) -> LP_GPIO7_EDGE_WAKEUP_CLR_W<PIN7_SPEC, 3> {
        LP_GPIO7_EDGE_WAKEUP_CLR_W::new(self)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio7_int_type(&mut self) -> LP_GPIO7_INT_TYPE_W<PIN7_SPEC, 7> {
        LP_GPIO7_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio7_wakeup_enable(&mut self) -> LP_GPIO7_WAKEUP_ENABLE_W<PIN7_SPEC, 10> {
        LP_GPIO7_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio7_filter_en(&mut self) -> LP_GPIO7_FILTER_EN_W<PIN7_SPEC, 11> {
        LP_GPIO7_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN7_SPEC;
impl crate::RegisterSpec for PIN7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin7::R`](R) reader structure"]
impl crate::Readable for PIN7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin7::W`](W) writer structure"]
impl crate::Writable for PIN7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN7 to value 0"]
impl crate::Resettable for PIN7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
