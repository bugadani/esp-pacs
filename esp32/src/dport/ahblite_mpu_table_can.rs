#[doc = "Register `AHBLITE_MPU_TABLE_CAN` reader"]
pub type R = crate::R<AHBLITE_MPU_TABLE_CAN_SPEC>;
#[doc = "Register `AHBLITE_MPU_TABLE_CAN` writer"]
pub type W = crate::W<AHBLITE_MPU_TABLE_CAN_SPEC>;
#[doc = "Field `CAN_ACCESS_GRANT_CONFIG` reader - "]
pub type CAN_ACCESS_GRANT_CONFIG_R = crate::FieldReader;
#[doc = "Field `CAN_ACCESS_GRANT_CONFIG` writer - "]
pub type CAN_ACCESS_GRANT_CONFIG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn can_access_grant_config(&self) -> CAN_ACCESS_GRANT_CONFIG_R {
        CAN_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLITE_MPU_TABLE_CAN")
            .field(
                "can_access_grant_config",
                &format_args!("{}", self.can_access_grant_config().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHBLITE_MPU_TABLE_CAN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn can_access_grant_config(
        &mut self,
    ) -> CAN_ACCESS_GRANT_CONFIG_W<AHBLITE_MPU_TABLE_CAN_SPEC, 0> {
        CAN_ACCESS_GRANT_CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_can::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_can::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLITE_MPU_TABLE_CAN_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_CAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblite_mpu_table_can::R`](R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_CAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblite_mpu_table_can::W`](W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_CAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBLITE_MPU_TABLE_CAN to value 0"]
impl crate::Resettable for AHBLITE_MPU_TABLE_CAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
