#[doc = "Register `CFG_DATA16` reader"]
pub type R = crate::R<CFG_DATA16_SPEC>;
#[doc = "Register `CFG_DATA16` writer"]
pub type W = crate::W<CFG_DATA16_SPEC>;
#[doc = "Field `USER_ID_FN2` reader - "]
pub type USER_ID_FN2_R = crate::FieldReader<u16>;
#[doc = "Field `USER_ID_FN2` writer - "]
pub type USER_ID_FN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DEVICE_ID_FN2` reader - "]
pub type DEVICE_ID_FN2_R = crate::FieldReader<u16>;
#[doc = "Field `DEVICE_ID_FN2` writer - "]
pub type DEVICE_ID_FN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn2(&self) -> USER_ID_FN2_R {
        USER_ID_FN2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn2(&self) -> DEVICE_ID_FN2_R {
        DEVICE_ID_FN2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA16")
            .field(
                "user_id_fn2",
                &format_args!("{}", self.user_id_fn2().bits()),
            )
            .field(
                "device_id_fn2",
                &format_args!("{}", self.device_id_fn2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_DATA16_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn user_id_fn2(&mut self) -> USER_ID_FN2_W<CFG_DATA16_SPEC, 0> {
        USER_ID_FN2_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn device_id_fn2(&mut self) -> DEVICE_ID_FN2_W<CFG_DATA16_SPEC, 16> {
        DEVICE_ID_FN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_DATA16_SPEC;
impl crate::RegisterSpec for CFG_DATA16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data16::R`](R) reader structure"]
impl crate::Readable for CFG_DATA16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_data16::W`](W) writer structure"]
impl crate::Writable for CFG_DATA16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DATA16 to value 0x3333_6666"]
impl crate::Resettable for CFG_DATA16_SPEC {
    const RESET_VALUE: Self::Ux = 0x3333_6666;
}
