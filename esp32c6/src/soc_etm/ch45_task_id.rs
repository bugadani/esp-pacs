#[doc = "Register `CH45_TASK_ID` reader"]
pub struct R(crate::R<CH45_TASK_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH45_TASK_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH45_TASK_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH45_TASK_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH45_TASK_ID` writer"]
pub struct W(crate::W<CH45_TASK_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH45_TASK_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH45_TASK_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH45_TASK_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH45_TASK_ID` reader - ch45_task_id"]
pub type CH45_TASK_ID_R = crate::FieldReader;
#[doc = "Field `CH45_TASK_ID` writer - ch45_task_id"]
pub type CH45_TASK_ID_W<'a, const O: u8> = crate::FieldWriter<'a, CH45_TASK_ID_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ch45_task_id"]
    #[inline(always)]
    pub fn ch45_task_id(&self) -> CH45_TASK_ID_R {
        CH45_TASK_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH45_TASK_ID")
            .field(
                "ch45_task_id",
                &format_args!("{}", self.ch45_task_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH45_TASK_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ch45_task_id"]
    #[inline(always)]
    #[must_use]
    pub fn ch45_task_id(&mut self) -> CH45_TASK_ID_W<0> {
        CH45_TASK_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel45 task id register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch45_task_id](index.html) module"]
pub struct CH45_TASK_ID_SPEC;
impl crate::RegisterSpec for CH45_TASK_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch45_task_id::R](R) reader structure"]
impl crate::Readable for CH45_TASK_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch45_task_id::W](W) writer structure"]
impl crate::Writable for CH45_TASK_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH45_TASK_ID to value 0"]
impl crate::Resettable for CH45_TASK_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
