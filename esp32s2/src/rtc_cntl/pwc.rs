#[doc = "Register `PWC` reader"]
pub type R = crate::R<PWC_SPEC>;
#[doc = "Register `PWC` writer"]
pub type W = crate::W<PWC_SPEC>;
#[doc = "Field `FASTMEM_FORCE_NOISO` reader - Set this bit to disable the force isolation to the RTC fast memory."]
pub type FASTMEM_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_NOISO` writer - Set this bit to disable the force isolation to the RTC fast memory."]
pub type FASTMEM_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FASTMEM_FORCE_ISO` reader - Set this bit to force isolate the RTC fast memory."]
pub type FASTMEM_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_ISO` writer - Set this bit to force isolate the RTC fast memory."]
pub type FASTMEM_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_FORCE_NOISO` reader - Set this bit to disable the force isolation to the RTC slow memory."]
pub type SLOWMEM_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_NOISO` writer - Set this bit to disable the force isolation to the RTC slow memory."]
pub type SLOWMEM_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_FORCE_ISO` reader - Set this bit to force isolate the RTC slow memory."]
pub type SLOWMEM_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_ISO` writer - Set this bit to force isolate the RTC slow memory."]
pub type SLOWMEM_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_ISO` reader - Set this bit to force isolate the RTC peripherals."]
pub type FORCE_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_ISO` writer - Set this bit to force isolate the RTC peripherals."]
pub type FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_NOISO` reader - Set this bit to disable the force isolation to the RTC peripherals."]
pub type FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `FORCE_NOISO` writer - Set this bit to disable the force isolation to the RTC peripherals."]
pub type FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FASTMEM_FOLW_CPU` reader - Set 1 to FPD the RTC fast memory when the CPU is powered down. Set 0 to FPD the RTC fast memory when the RTC main state machine is powered down."]
pub type FASTMEM_FOLW_CPU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FOLW_CPU` writer - Set 1 to FPD the RTC fast memory when the CPU is powered down. Set 0 to FPD the RTC fast memory when the RTC main state machine is powered down."]
pub type FASTMEM_FOLW_CPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FASTMEM_FORCE_LPD` reader - Set this bit to force not retain the RTC fast memory."]
pub type FASTMEM_FORCE_LPD_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_LPD` writer - Set this bit to force not retain the RTC fast memory."]
pub type FASTMEM_FORCE_LPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FASTMEM_FORCE_LPU` reader - Set this bit to force retain the RTC fast memory."]
pub type FASTMEM_FORCE_LPU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_LPU` writer - Set this bit to force retain the RTC fast memory."]
pub type FASTMEM_FORCE_LPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_FOLW_CPU` reader - Set 1 to FPD the RTC slow memory when the CPU is powered down. Set 0 to FPD the RTC slow memory when the RTC main state machine is powered down."]
pub type SLOWMEM_FOLW_CPU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FOLW_CPU` writer - Set 1 to FPD the RTC slow memory when the CPU is powered down. Set 0 to FPD the RTC slow memory when the RTC main state machine is powered down."]
pub type SLOWMEM_FOLW_CPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_FORCE_LPD` reader - Set this bit to force not retain the RTC slow memory."]
pub type SLOWMEM_FORCE_LPD_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_LPD` writer - Set this bit to force not retain the RTC slow memory."]
pub type SLOWMEM_FORCE_LPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_FORCE_LPU` reader - Set this bit to force retain the RTC slow memory."]
pub type SLOWMEM_FORCE_LPU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_LPU` writer - Set this bit to force retain the RTC slow memory."]
pub type SLOWMEM_FORCE_LPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FASTMEM_FORCE_PD` reader - Set this bit to FPD the RTC fast memory."]
pub type FASTMEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_PD` writer - Set this bit to FPD the RTC fast memory."]
pub type FASTMEM_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FASTMEM_FORCE_PU` reader - Set this bit to FPU the RTC fast memory."]
pub type FASTMEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_PU` writer - Set this bit to FPU the RTC fast memory."]
pub type FASTMEM_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FASTMEM_PD_EN` reader - Set this bit to enable PD for the RTC fast memory in sleep."]
pub type FASTMEM_PD_EN_R = crate::BitReader;
#[doc = "Field `FASTMEM_PD_EN` writer - Set this bit to enable PD for the RTC fast memory in sleep."]
pub type FASTMEM_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_FORCE_PD` reader - Set this bit to FPD the RTC slow memory."]
pub type SLOWMEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_PD` writer - Set this bit to FPD the RTC slow memory."]
pub type SLOWMEM_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_FORCE_PU` reader - Set this bit to FPU the RTC slow memory."]
pub type SLOWMEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_PU` writer - Set this bit to FPU the RTC slow memory."]
pub type SLOWMEM_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOWMEM_PD_EN` reader - Set this bit to enable PD for the RTC slow memory in sleep."]
pub type SLOWMEM_PD_EN_R = crate::BitReader;
#[doc = "Field `SLOWMEM_PD_EN` writer - Set this bit to enable PD for the RTC slow memory in sleep."]
pub type SLOWMEM_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_PD` reader - Set this bit to FPD the RTC peripherals."]
pub type FORCE_PD_R = crate::BitReader;
#[doc = "Field `FORCE_PD` writer - Set this bit to FPD the RTC peripherals."]
pub type FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_PU` reader - Set this bit to FPU the RTC peripherals."]
pub type FORCE_PU_R = crate::BitReader;
#[doc = "Field `FORCE_PU` writer - Set this bit to FPU the RTC peripherals."]
pub type FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD_EN` reader - Set this bit to enable PD for the RTC peripherals in sleep."]
pub type PD_EN_R = crate::BitReader;
#[doc = "Field `PD_EN` writer - Set this bit to enable PD for the RTC peripherals in sleep."]
pub type PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAD_FORCE_HOLD` reader - Set this bit the force hold the RTC GPIOs."]
pub type PAD_FORCE_HOLD_R = crate::BitReader;
#[doc = "Field `PAD_FORCE_HOLD` writer - Set this bit the force hold the RTC GPIOs."]
pub type PAD_FORCE_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to disable the force isolation to the RTC fast memory."]
    #[inline(always)]
    pub fn fastmem_force_noiso(&self) -> FASTMEM_FORCE_NOISO_R {
        FASTMEM_FORCE_NOISO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force isolate the RTC fast memory."]
    #[inline(always)]
    pub fn fastmem_force_iso(&self) -> FASTMEM_FORCE_ISO_R {
        FASTMEM_FORCE_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to disable the force isolation to the RTC slow memory."]
    #[inline(always)]
    pub fn slowmem_force_noiso(&self) -> SLOWMEM_FORCE_NOISO_R {
        SLOWMEM_FORCE_NOISO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force isolate the RTC slow memory."]
    #[inline(always)]
    pub fn slowmem_force_iso(&self) -> SLOWMEM_FORCE_ISO_R {
        SLOWMEM_FORCE_ISO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to force isolate the RTC peripherals."]
    #[inline(always)]
    pub fn force_iso(&self) -> FORCE_ISO_R {
        FORCE_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to disable the force isolation to the RTC peripherals."]
    #[inline(always)]
    pub fn force_noiso(&self) -> FORCE_NOISO_R {
        FORCE_NOISO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to FPD the RTC fast memory when the CPU is powered down. Set 0 to FPD the RTC fast memory when the RTC main state machine is powered down."]
    #[inline(always)]
    pub fn fastmem_folw_cpu(&self) -> FASTMEM_FOLW_CPU_R {
        FASTMEM_FOLW_CPU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to force not retain the RTC fast memory."]
    #[inline(always)]
    pub fn fastmem_force_lpd(&self) -> FASTMEM_FORCE_LPD_R {
        FASTMEM_FORCE_LPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to force retain the RTC fast memory."]
    #[inline(always)]
    pub fn fastmem_force_lpu(&self) -> FASTMEM_FORCE_LPU_R {
        FASTMEM_FORCE_LPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set 1 to FPD the RTC slow memory when the CPU is powered down. Set 0 to FPD the RTC slow memory when the RTC main state machine is powered down."]
    #[inline(always)]
    pub fn slowmem_folw_cpu(&self) -> SLOWMEM_FOLW_CPU_R {
        SLOWMEM_FOLW_CPU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to force not retain the RTC slow memory."]
    #[inline(always)]
    pub fn slowmem_force_lpd(&self) -> SLOWMEM_FORCE_LPD_R {
        SLOWMEM_FORCE_LPD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to force retain the RTC slow memory."]
    #[inline(always)]
    pub fn slowmem_force_lpu(&self) -> SLOWMEM_FORCE_LPU_R {
        SLOWMEM_FORCE_LPU_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to FPD the RTC fast memory."]
    #[inline(always)]
    pub fn fastmem_force_pd(&self) -> FASTMEM_FORCE_PD_R {
        FASTMEM_FORCE_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to FPU the RTC fast memory."]
    #[inline(always)]
    pub fn fastmem_force_pu(&self) -> FASTMEM_FORCE_PU_R {
        FASTMEM_FORCE_PU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable PD for the RTC fast memory in sleep."]
    #[inline(always)]
    pub fn fastmem_pd_en(&self) -> FASTMEM_PD_EN_R {
        FASTMEM_PD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to FPD the RTC slow memory."]
    #[inline(always)]
    pub fn slowmem_force_pd(&self) -> SLOWMEM_FORCE_PD_R {
        SLOWMEM_FORCE_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to FPU the RTC slow memory."]
    #[inline(always)]
    pub fn slowmem_force_pu(&self) -> SLOWMEM_FORCE_PU_R {
        SLOWMEM_FORCE_PU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable PD for the RTC slow memory in sleep."]
    #[inline(always)]
    pub fn slowmem_pd_en(&self) -> SLOWMEM_PD_EN_R {
        SLOWMEM_PD_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to FPD the RTC peripherals."]
    #[inline(always)]
    pub fn force_pd(&self) -> FORCE_PD_R {
        FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to FPU the RTC peripherals."]
    #[inline(always)]
    pub fn force_pu(&self) -> FORCE_PU_R {
        FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable PD for the RTC peripherals in sleep."]
    #[inline(always)]
    pub fn pd_en(&self) -> PD_EN_R {
        PD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit the force hold the RTC GPIOs."]
    #[inline(always)]
    pub fn pad_force_hold(&self) -> PAD_FORCE_HOLD_R {
        PAD_FORCE_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWC")
            .field(
                "fastmem_force_noiso",
                &format_args!("{}", self.fastmem_force_noiso().bit()),
            )
            .field(
                "fastmem_force_iso",
                &format_args!("{}", self.fastmem_force_iso().bit()),
            )
            .field(
                "slowmem_force_noiso",
                &format_args!("{}", self.slowmem_force_noiso().bit()),
            )
            .field(
                "slowmem_force_iso",
                &format_args!("{}", self.slowmem_force_iso().bit()),
            )
            .field("force_iso", &format_args!("{}", self.force_iso().bit()))
            .field("force_noiso", &format_args!("{}", self.force_noiso().bit()))
            .field(
                "fastmem_folw_cpu",
                &format_args!("{}", self.fastmem_folw_cpu().bit()),
            )
            .field(
                "fastmem_force_lpd",
                &format_args!("{}", self.fastmem_force_lpd().bit()),
            )
            .field(
                "fastmem_force_lpu",
                &format_args!("{}", self.fastmem_force_lpu().bit()),
            )
            .field(
                "slowmem_folw_cpu",
                &format_args!("{}", self.slowmem_folw_cpu().bit()),
            )
            .field(
                "slowmem_force_lpd",
                &format_args!("{}", self.slowmem_force_lpd().bit()),
            )
            .field(
                "slowmem_force_lpu",
                &format_args!("{}", self.slowmem_force_lpu().bit()),
            )
            .field(
                "fastmem_force_pd",
                &format_args!("{}", self.fastmem_force_pd().bit()),
            )
            .field(
                "fastmem_force_pu",
                &format_args!("{}", self.fastmem_force_pu().bit()),
            )
            .field(
                "fastmem_pd_en",
                &format_args!("{}", self.fastmem_pd_en().bit()),
            )
            .field(
                "slowmem_force_pd",
                &format_args!("{}", self.slowmem_force_pd().bit()),
            )
            .field(
                "slowmem_force_pu",
                &format_args!("{}", self.slowmem_force_pu().bit()),
            )
            .field(
                "slowmem_pd_en",
                &format_args!("{}", self.slowmem_pd_en().bit()),
            )
            .field("force_pd", &format_args!("{}", self.force_pd().bit()))
            .field("force_pu", &format_args!("{}", self.force_pu().bit()))
            .field("pd_en", &format_args!("{}", self.pd_en().bit()))
            .field(
                "pad_force_hold",
                &format_args!("{}", self.pad_force_hold().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to disable the force isolation to the RTC fast memory."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_noiso(&mut self) -> FASTMEM_FORCE_NOISO_W<PWC_SPEC, 0> {
        FASTMEM_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to force isolate the RTC fast memory."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_iso(&mut self) -> FASTMEM_FORCE_ISO_W<PWC_SPEC, 1> {
        FASTMEM_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to disable the force isolation to the RTC slow memory."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_noiso(&mut self) -> SLOWMEM_FORCE_NOISO_W<PWC_SPEC, 2> {
        SLOWMEM_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to force isolate the RTC slow memory."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_iso(&mut self) -> SLOWMEM_FORCE_ISO_W<PWC_SPEC, 3> {
        SLOWMEM_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to force isolate the RTC peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn force_iso(&mut self) -> FORCE_ISO_W<PWC_SPEC, 4> {
        FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to disable the force isolation to the RTC peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn force_noiso(&mut self) -> FORCE_NOISO_W<PWC_SPEC, 5> {
        FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to FPD the RTC fast memory when the CPU is powered down. Set 0 to FPD the RTC fast memory when the RTC main state machine is powered down."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_folw_cpu(&mut self) -> FASTMEM_FOLW_CPU_W<PWC_SPEC, 6> {
        FASTMEM_FOLW_CPU_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to force not retain the RTC fast memory."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpd(&mut self) -> FASTMEM_FORCE_LPD_W<PWC_SPEC, 7> {
        FASTMEM_FORCE_LPD_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to force retain the RTC fast memory."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpu(&mut self) -> FASTMEM_FORCE_LPU_W<PWC_SPEC, 8> {
        FASTMEM_FORCE_LPU_W::new(self)
    }
    #[doc = "Bit 9 - Set 1 to FPD the RTC slow memory when the CPU is powered down. Set 0 to FPD the RTC slow memory when the RTC main state machine is powered down."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_folw_cpu(&mut self) -> SLOWMEM_FOLW_CPU_W<PWC_SPEC, 9> {
        SLOWMEM_FOLW_CPU_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to force not retain the RTC slow memory."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_lpd(&mut self) -> SLOWMEM_FORCE_LPD_W<PWC_SPEC, 10> {
        SLOWMEM_FORCE_LPD_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to force retain the RTC slow memory."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_lpu(&mut self) -> SLOWMEM_FORCE_LPU_W<PWC_SPEC, 11> {
        SLOWMEM_FORCE_LPU_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to FPD the RTC fast memory."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_pd(&mut self) -> FASTMEM_FORCE_PD_W<PWC_SPEC, 12> {
        FASTMEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to FPU the RTC fast memory."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_pu(&mut self) -> FASTMEM_FORCE_PU_W<PWC_SPEC, 13> {
        FASTMEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to enable PD for the RTC fast memory in sleep."]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_pd_en(&mut self) -> FASTMEM_PD_EN_W<PWC_SPEC, 14> {
        FASTMEM_PD_EN_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to FPD the RTC slow memory."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_pd(&mut self) -> SLOWMEM_FORCE_PD_W<PWC_SPEC, 15> {
        SLOWMEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to FPU the RTC slow memory."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_pu(&mut self) -> SLOWMEM_FORCE_PU_W<PWC_SPEC, 16> {
        SLOWMEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to enable PD for the RTC slow memory in sleep."]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_pd_en(&mut self) -> SLOWMEM_PD_EN_W<PWC_SPEC, 17> {
        SLOWMEM_PD_EN_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to FPD the RTC peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn force_pd(&mut self) -> FORCE_PD_W<PWC_SPEC, 18> {
        FORCE_PD_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to FPU the RTC peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn force_pu(&mut self) -> FORCE_PU_W<PWC_SPEC, 19> {
        FORCE_PU_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to enable PD for the RTC peripherals in sleep."]
    #[inline(always)]
    #[must_use]
    pub fn pd_en(&mut self) -> PD_EN_W<PWC_SPEC, 20> {
        PD_EN_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit the force hold the RTC GPIOs."]
    #[inline(always)]
    #[must_use]
    pub fn pad_force_hold(&mut self) -> PAD_FORCE_HOLD_W<PWC_SPEC, 21> {
        PAD_FORCE_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC power configuraiton register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWC_SPEC;
impl crate::RegisterSpec for PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwc::R`](R) reader structure"]
impl crate::Readable for PWC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwc::W`](W) writer structure"]
impl crate::Writable for PWC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWC to value 0x0001_2925"]
impl crate::Resettable for PWC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_2925;
}
