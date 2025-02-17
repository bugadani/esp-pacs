#[doc = "Register `MEM_TEST_CONF` reader"]
pub type R = crate::R<MEM_TEST_CONF_SPEC>;
#[doc = "Register `MEM_TEST_CONF` writer"]
pub type W = crate::W<MEM_TEST_CONF_SPEC>;
#[doc = "Field `HP_MEM_WPULSE` reader - This field controls hp system memory WPULSE parameter."]
pub type HP_MEM_WPULSE_R = crate::FieldReader;
#[doc = "Field `HP_MEM_WPULSE` writer - This field controls hp system memory WPULSE parameter."]
pub type HP_MEM_WPULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HP_MEM_WA` reader - This field controls hp system memory WA parameter."]
pub type HP_MEM_WA_R = crate::FieldReader;
#[doc = "Field `HP_MEM_WA` writer - This field controls hp system memory WA parameter."]
pub type HP_MEM_WA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HP_MEM_RA` reader - This field controls hp system memory RA parameter."]
pub type HP_MEM_RA_R = crate::FieldReader;
#[doc = "Field `HP_MEM_RA` writer - This field controls hp system memory RA parameter."]
pub type HP_MEM_RA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:2 - This field controls hp system memory WPULSE parameter."]
    #[inline(always)]
    pub fn hp_mem_wpulse(&self) -> HP_MEM_WPULSE_R {
        HP_MEM_WPULSE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - This field controls hp system memory WA parameter."]
    #[inline(always)]
    pub fn hp_mem_wa(&self) -> HP_MEM_WA_R {
        HP_MEM_WA_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - This field controls hp system memory RA parameter."]
    #[inline(always)]
    pub fn hp_mem_ra(&self) -> HP_MEM_RA_R {
        HP_MEM_RA_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_TEST_CONF")
            .field(
                "hp_mem_wpulse",
                &format_args!("{}", self.hp_mem_wpulse().bits()),
            )
            .field("hp_mem_wa", &format_args!("{}", self.hp_mem_wa().bits()))
            .field("hp_mem_ra", &format_args!("{}", self.hp_mem_ra().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_TEST_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - This field controls hp system memory WPULSE parameter."]
    #[inline(always)]
    #[must_use]
    pub fn hp_mem_wpulse(&mut self) -> HP_MEM_WPULSE_W<MEM_TEST_CONF_SPEC, 0> {
        HP_MEM_WPULSE_W::new(self)
    }
    #[doc = "Bits 3:5 - This field controls hp system memory WA parameter."]
    #[inline(always)]
    #[must_use]
    pub fn hp_mem_wa(&mut self) -> HP_MEM_WA_W<MEM_TEST_CONF_SPEC, 3> {
        HP_MEM_WA_W::new(self)
    }
    #[doc = "Bits 6:7 - This field controls hp system memory RA parameter."]
    #[inline(always)]
    #[must_use]
    pub fn hp_mem_ra(&mut self) -> HP_MEM_RA_W<MEM_TEST_CONF_SPEC, 6> {
        HP_MEM_RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MEM_TEST configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_test_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_test_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_TEST_CONF_SPEC;
impl crate::RegisterSpec for MEM_TEST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_test_conf::R`](R) reader structure"]
impl crate::Readable for MEM_TEST_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_test_conf::W`](W) writer structure"]
impl crate::Writable for MEM_TEST_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_TEST_CONF to value 0x20"]
impl crate::Resettable for MEM_TEST_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
