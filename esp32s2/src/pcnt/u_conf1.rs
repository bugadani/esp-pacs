#[doc = "Register `U%s_CONF1` reader"]
pub type R = crate::R<U_CONF1_SPEC>;
#[doc = "Register `U%s_CONF1` writer"]
pub type W = crate::W<U_CONF1_SPEC>;
#[doc = "Field `CNT_THRES0` reader - This register is used to configure the thres0 value for unit %s."]
pub type CNT_THRES0_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_THRES0` writer - This register is used to configure the thres0 value for unit %s."]
pub type CNT_THRES0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CNT_THRES1` reader - This register is used to configure the thres1 value for unit %s."]
pub type CNT_THRES1_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_THRES1` writer - This register is used to configure the thres1 value for unit %s."]
pub type CNT_THRES1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the thres0 value for unit %s."]
    #[inline(always)]
    pub fn cnt_thres0(&self) -> CNT_THRES0_R {
        CNT_THRES0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure the thres1 value for unit %s."]
    #[inline(always)]
    pub fn cnt_thres1(&self) -> CNT_THRES1_R {
        CNT_THRES1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CONF1")
            .field("cnt_thres0", &format_args!("{}", self.cnt_thres0().bits()))
            .field("cnt_thres1", &format_args!("{}", self.cnt_thres1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the thres0 value for unit %s."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thres0(&mut self) -> CNT_THRES0_W<U_CONF1_SPEC, 0> {
        CNT_THRES0_W::new(self)
    }
    #[doc = "Bits 16:31 - This register is used to configure the thres1 value for unit %s."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thres1(&mut self) -> CNT_THRES1_W<U_CONF1_SPEC, 16> {
        CNT_THRES1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration register 1 for unit %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_CONF1_SPEC;
impl crate::RegisterSpec for U_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_conf1::R`](R) reader structure"]
impl crate::Readable for U_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u_conf1::W`](W) writer structure"]
impl crate::Writable for U_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets U%s_CONF1 to value 0"]
impl crate::Resettable for U_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
