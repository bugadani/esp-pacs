#[doc = "Register `MEM_MONITOR_CONF` reader"]
pub type R = crate::R<MEM_MONITOR_CONF_SPEC>;
#[doc = "Register `MEM_MONITOR_CONF` writer"]
pub type W = crate::W<MEM_MONITOR_CONF_SPEC>;
#[doc = "Field `MEM_MONITOR_CLK_EN` reader - Set 1 to enable mem_monitor clock"]
pub type MEM_MONITOR_CLK_EN_R = crate::BitReader;
#[doc = "Field `MEM_MONITOR_CLK_EN` writer - Set 1 to enable mem_monitor clock"]
pub type MEM_MONITOR_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEM_MONITOR_RST_EN` reader - Set 0 to reset mem_monitor module"]
pub type MEM_MONITOR_RST_EN_R = crate::BitReader;
#[doc = "Field `MEM_MONITOR_RST_EN` writer - Set 0 to reset mem_monitor module"]
pub type MEM_MONITOR_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEM_MONITOR_READY` reader - Query this field after reset mem_monitor module"]
pub type MEM_MONITOR_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable mem_monitor clock"]
    #[inline(always)]
    pub fn mem_monitor_clk_en(&self) -> MEM_MONITOR_CLK_EN_R {
        MEM_MONITOR_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset mem_monitor module"]
    #[inline(always)]
    pub fn mem_monitor_rst_en(&self) -> MEM_MONITOR_RST_EN_R {
        MEM_MONITOR_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset mem_monitor module"]
    #[inline(always)]
    pub fn mem_monitor_ready(&self) -> MEM_MONITOR_READY_R {
        MEM_MONITOR_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_MONITOR_CONF")
            .field(
                "mem_monitor_clk_en",
                &format_args!("{}", self.mem_monitor_clk_en().bit()),
            )
            .field(
                "mem_monitor_rst_en",
                &format_args!("{}", self.mem_monitor_rst_en().bit()),
            )
            .field(
                "mem_monitor_ready",
                &format_args!("{}", self.mem_monitor_ready().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_MONITOR_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable mem_monitor clock"]
    #[inline(always)]
    #[must_use]
    pub fn mem_monitor_clk_en(&mut self) -> MEM_MONITOR_CLK_EN_W<MEM_MONITOR_CONF_SPEC, 0> {
        MEM_MONITOR_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset mem_monitor module"]
    #[inline(always)]
    #[must_use]
    pub fn mem_monitor_rst_en(&mut self) -> MEM_MONITOR_RST_EN_W<MEM_MONITOR_CONF_SPEC, 1> {
        MEM_MONITOR_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MEM_MONITOR configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_monitor_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_monitor_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_MONITOR_CONF_SPEC;
impl crate::RegisterSpec for MEM_MONITOR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_monitor_conf::R`](R) reader structure"]
impl crate::Readable for MEM_MONITOR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_monitor_conf::W`](W) writer structure"]
impl crate::Writable for MEM_MONITOR_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_MONITOR_CONF to value 0x05"]
impl crate::Resettable for MEM_MONITOR_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
