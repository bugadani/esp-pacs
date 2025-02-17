#[doc = "Register `IRAM_DRAM_AHB_SEL` reader"]
pub type R = crate::R<IRAM_DRAM_AHB_SEL_SPEC>;
#[doc = "Register `IRAM_DRAM_AHB_SEL` writer"]
pub type W = crate::W<IRAM_DRAM_AHB_SEL_SPEC>;
#[doc = "Field `MASK_PRO_IRAM` reader - "]
pub type MASK_PRO_IRAM_R = crate::BitReader;
#[doc = "Field `MASK_PRO_IRAM` writer - "]
pub type MASK_PRO_IRAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK_APP_IRAM` reader - "]
pub type MASK_APP_IRAM_R = crate::BitReader;
#[doc = "Field `MASK_APP_IRAM` writer - "]
pub type MASK_APP_IRAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK_PRO_DRAM` reader - "]
pub type MASK_PRO_DRAM_R = crate::BitReader;
#[doc = "Field `MASK_PRO_DRAM` writer - "]
pub type MASK_PRO_DRAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK_APP_DRAM` reader - "]
pub type MASK_APP_DRAM_R = crate::BitReader;
#[doc = "Field `MASK_APP_DRAM` writer - "]
pub type MASK_APP_DRAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK_AHB` reader - "]
pub type MASK_AHB_R = crate::BitReader;
#[doc = "Field `MASK_AHB` writer - "]
pub type MASK_AHB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAC_DUMP_MODE` reader - "]
pub type MAC_DUMP_MODE_R = crate::FieldReader;
#[doc = "Field `MAC_DUMP_MODE` writer - "]
pub type MAC_DUMP_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mask_pro_iram(&self) -> MASK_PRO_IRAM_R {
        MASK_PRO_IRAM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mask_app_iram(&self) -> MASK_APP_IRAM_R {
        MASK_APP_IRAM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mask_pro_dram(&self) -> MASK_PRO_DRAM_R {
        MASK_PRO_DRAM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mask_app_dram(&self) -> MASK_APP_DRAM_R {
        MASK_APP_DRAM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mask_ahb(&self) -> MASK_AHB_R {
        MASK_AHB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn mac_dump_mode(&self) -> MAC_DUMP_MODE_R {
        MAC_DUMP_MODE_R::new(((self.bits >> 5) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRAM_DRAM_AHB_SEL")
            .field(
                "mask_pro_iram",
                &format_args!("{}", self.mask_pro_iram().bit()),
            )
            .field(
                "mask_app_iram",
                &format_args!("{}", self.mask_app_iram().bit()),
            )
            .field(
                "mask_pro_dram",
                &format_args!("{}", self.mask_pro_dram().bit()),
            )
            .field(
                "mask_app_dram",
                &format_args!("{}", self.mask_app_dram().bit()),
            )
            .field("mask_ahb", &format_args!("{}", self.mask_ahb().bit()))
            .field(
                "mac_dump_mode",
                &format_args!("{}", self.mac_dump_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IRAM_DRAM_AHB_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mask_pro_iram(&mut self) -> MASK_PRO_IRAM_W<IRAM_DRAM_AHB_SEL_SPEC, 0> {
        MASK_PRO_IRAM_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mask_app_iram(&mut self) -> MASK_APP_IRAM_W<IRAM_DRAM_AHB_SEL_SPEC, 1> {
        MASK_APP_IRAM_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn mask_pro_dram(&mut self) -> MASK_PRO_DRAM_W<IRAM_DRAM_AHB_SEL_SPEC, 2> {
        MASK_PRO_DRAM_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn mask_app_dram(&mut self) -> MASK_APP_DRAM_W<IRAM_DRAM_AHB_SEL_SPEC, 3> {
        MASK_APP_DRAM_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn mask_ahb(&mut self) -> MASK_AHB_W<IRAM_DRAM_AHB_SEL_SPEC, 4> {
        MASK_AHB_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn mac_dump_mode(&mut self) -> MAC_DUMP_MODE_W<IRAM_DRAM_AHB_SEL_SPEC, 5> {
        MAC_DUMP_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iram_dram_ahb_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iram_dram_ahb_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRAM_DRAM_AHB_SEL_SPEC;
impl crate::RegisterSpec for IRAM_DRAM_AHB_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iram_dram_ahb_sel::R`](R) reader structure"]
impl crate::Readable for IRAM_DRAM_AHB_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iram_dram_ahb_sel::W`](W) writer structure"]
impl crate::Writable for IRAM_DRAM_AHB_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRAM_DRAM_AHB_SEL to value 0"]
impl crate::Resettable for IRAM_DRAM_AHB_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
