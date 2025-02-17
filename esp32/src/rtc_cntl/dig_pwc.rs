#[doc = "Register `DIG_PWC` reader"]
pub type R = crate::R<DIG_PWC_SPEC>;
#[doc = "Register `DIG_PWC` writer"]
pub type W = crate::W<DIG_PWC_SPEC>;
#[doc = "Field `LSLP_MEM_FORCE_PD` reader - memories in digital core force PD in sleep"]
pub type LSLP_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PD` writer - memories in digital core force PD in sleep"]
pub type LSLP_MEM_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSLP_MEM_FORCE_PU` reader - memories in digital core force no PD in sleep"]
pub type LSLP_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PU` writer - memories in digital core force no PD in sleep"]
pub type LSLP_MEM_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROM0_FORCE_PD` reader - ROM force power down"]
pub type ROM0_FORCE_PD_R = crate::BitReader;
#[doc = "Field `ROM0_FORCE_PD` writer - ROM force power down"]
pub type ROM0_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROM0_FORCE_PU` reader - ROM force power up"]
pub type ROM0_FORCE_PU_R = crate::BitReader;
#[doc = "Field `ROM0_FORCE_PU` writer - ROM force power up"]
pub type ROM0_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM0_FORCE_PD` reader - internal SRAM 0 force power down"]
pub type INTER_RAM0_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_FORCE_PD` writer - internal SRAM 0 force power down"]
pub type INTER_RAM0_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM0_FORCE_PU` reader - internal SRAM 0 force power up"]
pub type INTER_RAM0_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_FORCE_PU` writer - internal SRAM 0 force power up"]
pub type INTER_RAM0_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM1_FORCE_PD` reader - internal SRAM 1 force power down"]
pub type INTER_RAM1_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_FORCE_PD` writer - internal SRAM 1 force power down"]
pub type INTER_RAM1_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM1_FORCE_PU` reader - internal SRAM 1 force power up"]
pub type INTER_RAM1_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_FORCE_PU` writer - internal SRAM 1 force power up"]
pub type INTER_RAM1_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM2_FORCE_PD` reader - internal SRAM 2 force power down"]
pub type INTER_RAM2_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_FORCE_PD` writer - internal SRAM 2 force power down"]
pub type INTER_RAM2_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM2_FORCE_PU` reader - internal SRAM 2 force power up"]
pub type INTER_RAM2_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_FORCE_PU` writer - internal SRAM 2 force power up"]
pub type INTER_RAM2_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM3_FORCE_PD` reader - internal SRAM 3 force power down"]
pub type INTER_RAM3_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_FORCE_PD` writer - internal SRAM 3 force power down"]
pub type INTER_RAM3_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM3_FORCE_PU` reader - internal SRAM 3 force power up"]
pub type INTER_RAM3_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_FORCE_PU` writer - internal SRAM 3 force power up"]
pub type INTER_RAM3_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM4_FORCE_PD` reader - internal SRAM 4 force power down"]
pub type INTER_RAM4_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_FORCE_PD` writer - internal SRAM 4 force power down"]
pub type INTER_RAM4_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM4_FORCE_PU` reader - internal SRAM 4 force power up"]
pub type INTER_RAM4_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_FORCE_PU` writer - internal SRAM 4 force power up"]
pub type INTER_RAM4_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIFI_FORCE_PD` reader - wifi force power down"]
pub type WIFI_FORCE_PD_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_PD` writer - wifi force power down"]
pub type WIFI_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIFI_FORCE_PU` reader - wifi force power up"]
pub type WIFI_FORCE_PU_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_PU` writer - wifi force power up"]
pub type WIFI_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_WRAP_FORCE_PD` reader - digital core force power down"]
pub type DG_WRAP_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PD` writer - digital core force power down"]
pub type DG_WRAP_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_WRAP_FORCE_PU` reader - digital core force power up"]
pub type DG_WRAP_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PU` writer - digital core force power up"]
pub type DG_WRAP_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROM0_PD_EN` reader - enable power down ROM in sleep"]
pub type ROM0_PD_EN_R = crate::BitReader;
#[doc = "Field `ROM0_PD_EN` writer - enable power down ROM in sleep"]
pub type ROM0_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM0_PD_EN` reader - enable power down internal SRAM 0 in sleep"]
pub type INTER_RAM0_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_PD_EN` writer - enable power down internal SRAM 0 in sleep"]
pub type INTER_RAM0_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM1_PD_EN` reader - enable power down internal SRAM 1 in sleep"]
pub type INTER_RAM1_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_PD_EN` writer - enable power down internal SRAM 1 in sleep"]
pub type INTER_RAM1_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM2_PD_EN` reader - enable power down internal SRAM 2 in sleep"]
pub type INTER_RAM2_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_PD_EN` writer - enable power down internal SRAM 2 in sleep"]
pub type INTER_RAM2_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM3_PD_EN` reader - enable power down internal SRAM 3 in sleep"]
pub type INTER_RAM3_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_PD_EN` writer - enable power down internal SRAM 3 in sleep"]
pub type INTER_RAM3_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM4_PD_EN` reader - enable power down internal SRAM 4 in sleep"]
pub type INTER_RAM4_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_PD_EN` writer - enable power down internal SRAM 4 in sleep"]
pub type INTER_RAM4_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIFI_PD_EN` reader - enable power down wifi in sleep"]
pub type WIFI_PD_EN_R = crate::BitReader;
#[doc = "Field `WIFI_PD_EN` writer - enable power down wifi in sleep"]
pub type WIFI_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_WRAP_PD_EN` reader - enable power down digital core in sleep"]
pub type DG_WRAP_PD_EN_R = crate::BitReader;
#[doc = "Field `DG_WRAP_PD_EN` writer - enable power down digital core in sleep"]
pub type DG_WRAP_PD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - memories in digital core force PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    pub fn rom0_force_pd(&self) -> ROM0_FORCE_PD_R {
        ROM0_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    pub fn rom0_force_pu(&self) -> ROM0_FORCE_PU_R {
        ROM0_FORCE_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    pub fn inter_ram0_force_pd(&self) -> INTER_RAM0_FORCE_PD_R {
        INTER_RAM0_FORCE_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    pub fn inter_ram0_force_pu(&self) -> INTER_RAM0_FORCE_PU_R {
        INTER_RAM0_FORCE_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    pub fn inter_ram1_force_pd(&self) -> INTER_RAM1_FORCE_PD_R {
        INTER_RAM1_FORCE_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    pub fn inter_ram1_force_pu(&self) -> INTER_RAM1_FORCE_PU_R {
        INTER_RAM1_FORCE_PU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn inter_ram2_force_pd(&self) -> INTER_RAM2_FORCE_PD_R {
        INTER_RAM2_FORCE_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn inter_ram2_force_pu(&self) -> INTER_RAM2_FORCE_PU_R {
        INTER_RAM2_FORCE_PU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn inter_ram3_force_pd(&self) -> INTER_RAM3_FORCE_PD_R {
        INTER_RAM3_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn inter_ram3_force_pu(&self) -> INTER_RAM3_FORCE_PU_R {
        INTER_RAM3_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    pub fn inter_ram4_force_pd(&self) -> INTER_RAM4_FORCE_PD_R {
        INTER_RAM4_FORCE_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    pub fn inter_ram4_force_pu(&self) -> INTER_RAM4_FORCE_PU_R {
        INTER_RAM4_FORCE_PU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - wifi force power down"]
    #[inline(always)]
    pub fn wifi_force_pd(&self) -> WIFI_FORCE_PD_R {
        WIFI_FORCE_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - wifi force power up"]
    #[inline(always)]
    pub fn wifi_force_pu(&self) -> WIFI_FORCE_PU_R {
        WIFI_FORCE_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&self) -> DG_WRAP_FORCE_PD_R {
        DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&self) -> DG_WRAP_FORCE_PU_R {
        DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    pub fn rom0_pd_en(&self) -> ROM0_PD_EN_R {
        ROM0_PD_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    pub fn inter_ram0_pd_en(&self) -> INTER_RAM0_PD_EN_R {
        INTER_RAM0_PD_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    pub fn inter_ram1_pd_en(&self) -> INTER_RAM1_PD_EN_R {
        INTER_RAM1_PD_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn inter_ram2_pd_en(&self) -> INTER_RAM2_PD_EN_R {
        INTER_RAM2_PD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn inter_ram3_pd_en(&self) -> INTER_RAM3_PD_EN_R {
        INTER_RAM3_PD_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn inter_ram4_pd_en(&self) -> INTER_RAM4_PD_EN_R {
        INTER_RAM4_PD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable power down wifi in sleep"]
    #[inline(always)]
    pub fn wifi_pd_en(&self) -> WIFI_PD_EN_R {
        WIFI_PD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable power down digital core in sleep"]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&self) -> DG_WRAP_PD_EN_R {
        DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_PWC")
            .field(
                "lslp_mem_force_pd",
                &format_args!("{}", self.lslp_mem_force_pd().bit()),
            )
            .field(
                "lslp_mem_force_pu",
                &format_args!("{}", self.lslp_mem_force_pu().bit()),
            )
            .field(
                "rom0_force_pd",
                &format_args!("{}", self.rom0_force_pd().bit()),
            )
            .field(
                "rom0_force_pu",
                &format_args!("{}", self.rom0_force_pu().bit()),
            )
            .field(
                "inter_ram0_force_pd",
                &format_args!("{}", self.inter_ram0_force_pd().bit()),
            )
            .field(
                "inter_ram0_force_pu",
                &format_args!("{}", self.inter_ram0_force_pu().bit()),
            )
            .field(
                "inter_ram1_force_pd",
                &format_args!("{}", self.inter_ram1_force_pd().bit()),
            )
            .field(
                "inter_ram1_force_pu",
                &format_args!("{}", self.inter_ram1_force_pu().bit()),
            )
            .field(
                "inter_ram2_force_pd",
                &format_args!("{}", self.inter_ram2_force_pd().bit()),
            )
            .field(
                "inter_ram2_force_pu",
                &format_args!("{}", self.inter_ram2_force_pu().bit()),
            )
            .field(
                "inter_ram3_force_pd",
                &format_args!("{}", self.inter_ram3_force_pd().bit()),
            )
            .field(
                "inter_ram3_force_pu",
                &format_args!("{}", self.inter_ram3_force_pu().bit()),
            )
            .field(
                "inter_ram4_force_pd",
                &format_args!("{}", self.inter_ram4_force_pd().bit()),
            )
            .field(
                "inter_ram4_force_pu",
                &format_args!("{}", self.inter_ram4_force_pu().bit()),
            )
            .field(
                "wifi_force_pd",
                &format_args!("{}", self.wifi_force_pd().bit()),
            )
            .field(
                "wifi_force_pu",
                &format_args!("{}", self.wifi_force_pu().bit()),
            )
            .field(
                "dg_wrap_force_pd",
                &format_args!("{}", self.dg_wrap_force_pd().bit()),
            )
            .field(
                "dg_wrap_force_pu",
                &format_args!("{}", self.dg_wrap_force_pu().bit()),
            )
            .field("rom0_pd_en", &format_args!("{}", self.rom0_pd_en().bit()))
            .field(
                "inter_ram0_pd_en",
                &format_args!("{}", self.inter_ram0_pd_en().bit()),
            )
            .field(
                "inter_ram1_pd_en",
                &format_args!("{}", self.inter_ram1_pd_en().bit()),
            )
            .field(
                "inter_ram2_pd_en",
                &format_args!("{}", self.inter_ram2_pd_en().bit()),
            )
            .field(
                "inter_ram3_pd_en",
                &format_args!("{}", self.inter_ram3_pd_en().bit()),
            )
            .field(
                "inter_ram4_pd_en",
                &format_args!("{}", self.inter_ram4_pd_en().bit()),
            )
            .field("wifi_pd_en", &format_args!("{}", self.wifi_pd_en().bit()))
            .field(
                "dg_wrap_pd_en",
                &format_args!("{}", self.dg_wrap_pd_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIG_PWC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - memories in digital core force PD in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W<DIG_PWC_SPEC, 3> {
        LSLP_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 4 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W<DIG_PWC_SPEC, 4> {
        LSLP_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_force_pd(&mut self) -> ROM0_FORCE_PD_W<DIG_PWC_SPEC, 5> {
        ROM0_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_force_pu(&mut self) -> ROM0_FORCE_PU_W<DIG_PWC_SPEC, 6> {
        ROM0_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_force_pd(&mut self) -> INTER_RAM0_FORCE_PD_W<DIG_PWC_SPEC, 7> {
        INTER_RAM0_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_force_pu(&mut self) -> INTER_RAM0_FORCE_PU_W<DIG_PWC_SPEC, 8> {
        INTER_RAM0_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_force_pd(&mut self) -> INTER_RAM1_FORCE_PD_W<DIG_PWC_SPEC, 9> {
        INTER_RAM1_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_force_pu(&mut self) -> INTER_RAM1_FORCE_PU_W<DIG_PWC_SPEC, 10> {
        INTER_RAM1_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_force_pd(&mut self) -> INTER_RAM2_FORCE_PD_W<DIG_PWC_SPEC, 11> {
        INTER_RAM2_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_force_pu(&mut self) -> INTER_RAM2_FORCE_PU_W<DIG_PWC_SPEC, 12> {
        INTER_RAM2_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_force_pd(&mut self) -> INTER_RAM3_FORCE_PD_W<DIG_PWC_SPEC, 13> {
        INTER_RAM3_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_force_pu(&mut self) -> INTER_RAM3_FORCE_PU_W<DIG_PWC_SPEC, 14> {
        INTER_RAM3_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_force_pd(&mut self) -> INTER_RAM4_FORCE_PD_W<DIG_PWC_SPEC, 15> {
        INTER_RAM4_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_force_pu(&mut self) -> INTER_RAM4_FORCE_PU_W<DIG_PWC_SPEC, 16> {
        INTER_RAM4_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 17 - wifi force power down"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_pd(&mut self) -> WIFI_FORCE_PD_W<DIG_PWC_SPEC, 17> {
        WIFI_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 18 - wifi force power up"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_pu(&mut self) -> WIFI_FORCE_PU_W<DIG_PWC_SPEC, 18> {
        WIFI_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pd(&mut self) -> DG_WRAP_FORCE_PD_W<DIG_PWC_SPEC, 19> {
        DG_WRAP_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pu(&mut self) -> DG_WRAP_FORCE_PU_W<DIG_PWC_SPEC, 20> {
        DG_WRAP_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_pd_en(&mut self) -> ROM0_PD_EN_W<DIG_PWC_SPEC, 24> {
        ROM0_PD_EN_W::new(self)
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_pd_en(&mut self) -> INTER_RAM0_PD_EN_W<DIG_PWC_SPEC, 25> {
        INTER_RAM0_PD_EN_W::new(self)
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_pd_en(&mut self) -> INTER_RAM1_PD_EN_W<DIG_PWC_SPEC, 26> {
        INTER_RAM1_PD_EN_W::new(self)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_pd_en(&mut self) -> INTER_RAM2_PD_EN_W<DIG_PWC_SPEC, 27> {
        INTER_RAM2_PD_EN_W::new(self)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_pd_en(&mut self) -> INTER_RAM3_PD_EN_W<DIG_PWC_SPEC, 28> {
        INTER_RAM3_PD_EN_W::new(self)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_pd_en(&mut self) -> INTER_RAM4_PD_EN_W<DIG_PWC_SPEC, 29> {
        INTER_RAM4_PD_EN_W::new(self)
    }
    #[doc = "Bit 30 - enable power down wifi in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_pd_en(&mut self) -> WIFI_PD_EN_W<DIG_PWC_SPEC, 30> {
        WIFI_PD_EN_W::new(self)
    }
    #[doc = "Bit 31 - enable power down digital core in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W<DIG_PWC_SPEC, 31> {
        DG_WRAP_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIG_PWC_SPEC;
impl crate::RegisterSpec for DIG_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_pwc::R`](R) reader structure"]
impl crate::Readable for DIG_PWC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dig_pwc::W`](W) writer structure"]
impl crate::Writable for DIG_PWC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIG_PWC to value 0x0015_5550"]
impl crate::Resettable for DIG_PWC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0015_5550;
}
