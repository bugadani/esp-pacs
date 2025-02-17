#[doc = "Register `CORE_X_IRAM0_PMS_CONSTRAIN_1` reader"]
pub type R = crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Register `CORE_X_IRAM0_PMS_CONSTRAIN_1` writer"]
pub type W = crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` reader - core0/core1's permission of instruction region0 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` writer - core0/core1's permission of instruction region0 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` reader - core0/core1's permission of instruction region1 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` writer - core0/core1's permission of instruction region1 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` reader - core0/core1's permission of instruction region2 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` writer - core0/core1's permission of instruction region2 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` reader - core0/core1's permission of instruction region3 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` writer - core0/core1's permission of instruction region3 of SRAM in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0` reader - core0/core1's permission of icache data sram block0 in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0` writer - core0/core1's permission of icache data sram block0 in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1` reader - core0/core1's permission of icache data sram block1 in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1` writer - core0/core1's permission of icache data sram block1 in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS` reader - core0/core1's permission of rom in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS` writer - core0/core1's permission of rom in world1"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - core0/core1's permission of instruction region0 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_0(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - core0/core1's permission of instruction region1 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_1(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - core0/core1's permission of instruction region2 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_2(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - core0/core1's permission of instruction region3 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_3(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - core0/core1's permission of icache data sram block0 in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_0(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0_R::new(
            ((self.bits >> 12) & 7) as u8,
        )
    }
    #[doc = "Bits 15:17 - core0/core1's permission of icache data sram block1 in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_1(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1_R::new(
            ((self.bits >> 15) & 7) as u8,
        )
    }
    #[doc = "Bits 18:20 - core0/core1's permission of rom in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_rom_world_1_pms(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R::new(((self.bits >> 18) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_PMS_CONSTRAIN_1")
            .field(
                "core_x_iram0_pms_constrain_sram_world_1_pms_0",
                &format_args!(
                    "{}",
                    self.core_x_iram0_pms_constrain_sram_world_1_pms_0().bits()
                ),
            )
            .field(
                "core_x_iram0_pms_constrain_sram_world_1_pms_1",
                &format_args!(
                    "{}",
                    self.core_x_iram0_pms_constrain_sram_world_1_pms_1().bits()
                ),
            )
            .field(
                "core_x_iram0_pms_constrain_sram_world_1_pms_2",
                &format_args!(
                    "{}",
                    self.core_x_iram0_pms_constrain_sram_world_1_pms_2().bits()
                ),
            )
            .field(
                "core_x_iram0_pms_constrain_sram_world_1_pms_3",
                &format_args!(
                    "{}",
                    self.core_x_iram0_pms_constrain_sram_world_1_pms_3().bits()
                ),
            )
            .field(
                "core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_0",
                &format_args!(
                    "{}",
                    self.core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_0()
                        .bits()
                ),
            )
            .field(
                "core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_1",
                &format_args!(
                    "{}",
                    self.core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_1()
                        .bits()
                ),
            )
            .field(
                "core_x_iram0_pms_constrain_rom_world_1_pms",
                &format_args!(
                    "{}",
                    self.core_x_iram0_pms_constrain_rom_world_1_pms().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - core0/core1's permission of instruction region0 of SRAM in world1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_0(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC, 0> {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W::new(self)
    }
    #[doc = "Bits 3:5 - core0/core1's permission of instruction region1 of SRAM in world1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_1(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC, 3> {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W::new(self)
    }
    #[doc = "Bits 6:8 - core0/core1's permission of instruction region2 of SRAM in world1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_2(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC, 6> {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W::new(self)
    }
    #[doc = "Bits 9:11 - core0/core1's permission of instruction region3 of SRAM in world1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_sram_world_1_pms_3(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC, 9> {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W::new(self)
    }
    #[doc = "Bits 12:14 - core0/core1's permission of icache data sram block0 in world1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_0(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0_W<
        CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC,
        12,
    > {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0_W::new(self)
    }
    #[doc = "Bits 15:17 - core0/core1's permission of icache data sram block1 in world1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_1(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1_W<
        CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC,
        15,
    > {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_1_W::new(self)
    }
    #[doc = "Bits 18:20 - core0/core1's permission of rom in world1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_rom_world_1_pms(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_W<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC, 18> {
        CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "corex iram0 permission configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_pms_constrain_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_pms_constrain_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_iram0_pms_constrain_1::R`](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_x_iram0_pms_constrain_1::W`](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_PMS_CONSTRAIN_1 to value 0x001f_ffff"]
impl crate::Resettable for CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x001f_ffff;
}
