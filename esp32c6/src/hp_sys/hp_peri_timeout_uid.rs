#[doc = "Register `HP_PERI_TIMEOUT_UID` reader"]
pub struct R(crate::R<HP_PERI_TIMEOUT_UID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_PERI_TIMEOUT_UID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_PERI_TIMEOUT_UID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_PERI_TIMEOUT_UID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HP_PERI_TIMEOUT_UID` reader - Record master id\\[4:0\\] &amp; master permission\\[6:5\\] when trigger timeout. This register will be cleared after the interrupt is cleared."]
pub type HP_PERI_TIMEOUT_UID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Record master id\\[4:0\\] &amp; master permission\\[6:5\\] when trigger timeout. This register will be cleared after the interrupt is cleared."]
    #[inline(always)]
    pub fn hp_peri_timeout_uid(&self) -> HP_PERI_TIMEOUT_UID_R {
        HP_PERI_TIMEOUT_UID_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "HP_PERI_TIMEOUT_UID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_peri_timeout_uid](index.html) module"]
pub struct HP_PERI_TIMEOUT_UID_SPEC;
impl crate::RegisterSpec for HP_PERI_TIMEOUT_UID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_peri_timeout_uid::R](R) reader structure"]
impl crate::Readable for HP_PERI_TIMEOUT_UID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HP_PERI_TIMEOUT_UID to value 0"]
impl crate::Resettable for HP_PERI_TIMEOUT_UID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
