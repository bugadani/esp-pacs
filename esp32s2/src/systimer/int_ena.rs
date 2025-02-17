#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TARGET0_INT_ENA` reader - Interrupt enable bit of system timer target 0."]
pub type TARGET0_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET0_INT_ENA` writer - Interrupt enable bit of system timer target 0."]
pub type TARGET0_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET1_INT_ENA` reader - Interrupt enable bit of system timer target 1."]
pub type TARGET1_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET1_INT_ENA` writer - Interrupt enable bit of system timer target 1."]
pub type TARGET1_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET2_INT_ENA` reader - Interrupt enable bit of system timer target 2."]
pub type TARGET2_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET2_INT_ENA` writer - Interrupt enable bit of system timer target 2."]
pub type TARGET2_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt enable bit of system timer target 0."]
    #[inline(always)]
    pub fn target0_int_ena(&self) -> TARGET0_INT_ENA_R {
        TARGET0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable bit of system timer target 1."]
    #[inline(always)]
    pub fn target1_int_ena(&self) -> TARGET1_INT_ENA_R {
        TARGET1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable bit of system timer target 2."]
    #[inline(always)]
    pub fn target2_int_ena(&self) -> TARGET2_INT_ENA_R {
        TARGET2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "target0_int_ena",
                &format_args!("{}", self.target0_int_ena().bit()),
            )
            .field(
                "target1_int_ena",
                &format_args!("{}", self.target1_int_ena().bit()),
            )
            .field(
                "target2_int_ena",
                &format_args!("{}", self.target2_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable bit of system timer target 0."]
    #[inline(always)]
    #[must_use]
    pub fn target0_int_ena(&mut self) -> TARGET0_INT_ENA_W<INT_ENA_SPEC, 0> {
        TARGET0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable bit of system timer target 1."]
    #[inline(always)]
    #[must_use]
    pub fn target1_int_ena(&mut self) -> TARGET1_INT_ENA_W<INT_ENA_SPEC, 1> {
        TARGET1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable bit of system timer target 2."]
    #[inline(always)]
    #[must_use]
    pub fn target2_int_ena(&mut self) -> TARGET2_INT_ENA_W<INT_ENA_SPEC, 2> {
        TARGET2_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System timer interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
