#[doc = "Register `Core_1_World_IRam0` reader"]
pub type R = crate::R<CORE_1_WORLD_IRAM0_SPEC>;
#[doc = "Register `Core_1_World_IRam0` writer"]
pub type W = crate::W<CORE_1_WORLD_IRAM0_SPEC>;
#[doc = "Field `CORE_1_WORLD_IRAM0` reader - this field is used to read current world of Iram0 bus"]
pub type CORE_1_WORLD_IRAM0_R = crate::FieldReader;
#[doc = "Field `CORE_1_WORLD_IRAM0` writer - this field is used to read current world of Iram0 bus"]
pub type CORE_1_WORLD_IRAM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    pub fn core_1_world_iram0(&self) -> CORE_1_WORLD_IRAM0_R {
        CORE_1_WORLD_IRAM0_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_World_IRam0")
            .field(
                "core_1_world_iram0",
                &format_args!("{}", self.core_1_world_iram0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_WORLD_IRAM0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_world_iram0(&mut self) -> CORE_1_WORLD_IRAM0_W<CORE_1_WORLD_IRAM0_SPEC, 0> {
        CORE_1_WORLD_IRAM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core_1 Iram0 world register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_world_iram0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_world_iram0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_WORLD_IRAM0_SPEC;
impl crate::RegisterSpec for CORE_1_WORLD_IRAM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_world_iram0::R`](R) reader structure"]
impl crate::Readable for CORE_1_WORLD_IRAM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_world_iram0::W`](W) writer structure"]
impl crate::Writable for CORE_1_WORLD_IRAM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_World_IRam0 to value 0"]
impl crate::Resettable for CORE_1_WORLD_IRAM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
