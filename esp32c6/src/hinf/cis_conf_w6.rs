#[doc = "Register `CIS_CONF_W6` reader"]
pub type R = crate::R<CIS_CONF_W6_SPEC>;
#[doc = "Register `CIS_CONF_W6` writer"]
pub type W = crate::W<CIS_CONF_W6_SPEC>;
#[doc = "Field `CIS_CONF_W6` reader - Configure cis addr 63~60"]
pub type CIS_CONF_W6_R = crate::FieldReader<u32>;
#[doc = "Field `CIS_CONF_W6` writer - Configure cis addr 63~60"]
pub type CIS_CONF_W6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure cis addr 63~60"]
    #[inline(always)]
    pub fn cis_conf_w6(&self) -> CIS_CONF_W6_R {
        CIS_CONF_W6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIS_CONF_W6")
            .field(
                "cis_conf_w6",
                &format_args!("{}", self.cis_conf_w6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CIS_CONF_W6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure cis addr 63~60"]
    #[inline(always)]
    #[must_use]
    pub fn cis_conf_w6(&mut self) -> CIS_CONF_W6_W<CIS_CONF_W6_SPEC, 0> {
        CIS_CONF_W6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDIO cis configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf_w6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf_w6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIS_CONF_W6_SPEC;
impl crate::RegisterSpec for CIS_CONF_W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cis_conf_w6::R`](R) reader structure"]
impl crate::Readable for CIS_CONF_W6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cis_conf_w6::W`](W) writer structure"]
impl crate::Writable for CIS_CONF_W6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIS_CONF_W6 to value 0xffff_ffff"]
impl crate::Resettable for CIS_CONF_W6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
