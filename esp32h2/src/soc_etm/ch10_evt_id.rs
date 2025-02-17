#[doc = "Register `CH10_EVT_ID` reader"]
pub type R = crate::R<CH10_EVT_ID_SPEC>;
#[doc = "Register `CH10_EVT_ID` writer"]
pub type W = crate::W<CH10_EVT_ID_SPEC>;
#[doc = "Field `CH10_EVT_ID` reader - ch10_evt_id"]
pub type CH10_EVT_ID_R = crate::FieldReader;
#[doc = "Field `CH10_EVT_ID` writer - ch10_evt_id"]
pub type CH10_EVT_ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ch10_evt_id"]
    #[inline(always)]
    pub fn ch10_evt_id(&self) -> CH10_EVT_ID_R {
        CH10_EVT_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH10_EVT_ID")
            .field(
                "ch10_evt_id",
                &format_args!("{}", self.ch10_evt_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH10_EVT_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ch10_evt_id"]
    #[inline(always)]
    #[must_use]
    pub fn ch10_evt_id(&mut self) -> CH10_EVT_ID_W<CH10_EVT_ID_SPEC, 0> {
        CH10_EVT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "channel10 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_evt_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_evt_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH10_EVT_ID_SPEC;
impl crate::RegisterSpec for CH10_EVT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch10_evt_id::R`](R) reader structure"]
impl crate::Readable for CH10_EVT_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch10_evt_id::W`](W) writer structure"]
impl crate::Writable for CH10_EVT_ID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH10_EVT_ID to value 0"]
impl crate::Resettable for CH10_EVT_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
