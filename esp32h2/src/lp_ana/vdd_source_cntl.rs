#[doc = "Register `VDD_SOURCE_CNTL` reader"]
pub type R = crate::R<VDD_SOURCE_CNTL_SPEC>;
#[doc = "Register `VDD_SOURCE_CNTL` writer"]
pub type W = crate::W<VDD_SOURCE_CNTL_SPEC>;
#[doc = "Field `DETMODE_SEL` reader - need_des"]
pub type DETMODE_SEL_R = crate::FieldReader;
#[doc = "Field `DETMODE_SEL` writer - need_des"]
pub type DETMODE_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VGOOD_EVENT_RECORD` reader - need_des"]
pub type VGOOD_EVENT_RECORD_R = crate::FieldReader;
#[doc = "Field `VBAT_EVENT_RECORD_CLR` writer - need_des"]
pub type VBAT_EVENT_RECORD_CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BOD_SOURCE_ENA` reader - need_des"]
pub type BOD_SOURCE_ENA_R = crate::FieldReader;
#[doc = "Field `BOD_SOURCE_ENA` writer - need_des"]
pub type BOD_SOURCE_ENA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn detmode_sel(&self) -> DETMODE_SEL_R {
        DETMODE_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn vgood_event_record(&self) -> VGOOD_EVENT_RECORD_R {
        VGOOD_EVENT_RECORD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn bod_source_ena(&self) -> BOD_SOURCE_ENA_R {
        BOD_SOURCE_ENA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDD_SOURCE_CNTL")
            .field(
                "detmode_sel",
                &format_args!("{}", self.detmode_sel().bits()),
            )
            .field(
                "vgood_event_record",
                &format_args!("{}", self.vgood_event_record().bits()),
            )
            .field(
                "bod_source_ena",
                &format_args!("{}", self.bod_source_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VDD_SOURCE_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn detmode_sel(&mut self) -> DETMODE_SEL_W<VDD_SOURCE_CNTL_SPEC, 0> {
        DETMODE_SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vbat_event_record_clr(&mut self) -> VBAT_EVENT_RECORD_CLR_W<VDD_SOURCE_CNTL_SPEC, 16> {
        VBAT_EVENT_RECORD_CLR_W::new(self)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_source_ena(&mut self) -> BOD_SOURCE_ENA_W<VDD_SOURCE_CNTL_SPEC, 24> {
        BOD_SOURCE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_source_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_source_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDD_SOURCE_CNTL_SPEC;
impl crate::RegisterSpec for VDD_SOURCE_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_source_cntl::R`](R) reader structure"]
impl crate::Readable for VDD_SOURCE_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdd_source_cntl::W`](W) writer structure"]
impl crate::Writable for VDD_SOURCE_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDD_SOURCE_CNTL to value 0x0400_00ff"]
impl crate::Resettable for VDD_SOURCE_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_00ff;
}
