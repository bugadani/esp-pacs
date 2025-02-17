#[doc = "Register `RX_TIMING` reader"]
pub type R = crate::R<RX_TIMING_SPEC>;
#[doc = "Register `RX_TIMING` writer"]
pub type W = crate::W<RX_TIMING_SPEC>;
#[doc = "Field `RX_SD_IN_DM` reader - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_SD_IN_DM_R = crate::FieldReader;
#[doc = "Field `RX_SD_IN_DM` writer - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_SD_IN_DM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RX_WS_OUT_DM` reader - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_WS_OUT_DM_R = crate::FieldReader;
#[doc = "Field `RX_WS_OUT_DM` writer - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_WS_OUT_DM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RX_BCK_OUT_DM` reader - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_BCK_OUT_DM_R = crate::FieldReader;
#[doc = "Field `RX_BCK_OUT_DM` writer - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_BCK_OUT_DM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RX_WS_IN_DM` reader - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_WS_IN_DM_R = crate::FieldReader;
#[doc = "Field `RX_WS_IN_DM` writer - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_WS_IN_DM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RX_BCK_IN_DM` reader - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_BCK_IN_DM_R = crate::FieldReader;
#[doc = "Field `RX_BCK_IN_DM` writer - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RX_BCK_IN_DM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd_in_dm(&self) -> RX_SD_IN_DM_R {
        RX_SD_IN_DM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_ws_out_dm(&self) -> RX_WS_OUT_DM_R {
        RX_WS_OUT_DM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_bck_out_dm(&self) -> RX_BCK_OUT_DM_R {
        RX_BCK_OUT_DM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_ws_in_dm(&self) -> RX_WS_IN_DM_R {
        RX_WS_IN_DM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_bck_in_dm(&self) -> RX_BCK_IN_DM_R {
        RX_BCK_IN_DM_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_TIMING")
            .field(
                "rx_sd_in_dm",
                &format_args!("{}", self.rx_sd_in_dm().bits()),
            )
            .field(
                "rx_ws_out_dm",
                &format_args!("{}", self.rx_ws_out_dm().bits()),
            )
            .field(
                "rx_bck_out_dm",
                &format_args!("{}", self.rx_bck_out_dm().bits()),
            )
            .field(
                "rx_ws_in_dm",
                &format_args!("{}", self.rx_ws_in_dm().bits()),
            )
            .field(
                "rx_bck_in_dm",
                &format_args!("{}", self.rx_bck_in_dm().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_TIMING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sd_in_dm(&mut self) -> RX_SD_IN_DM_W<RX_TIMING_SPEC, 0> {
        RX_SD_IN_DM_W::new(self)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ws_out_dm(&mut self) -> RX_WS_OUT_DM_W<RX_TIMING_SPEC, 16> {
        RX_WS_OUT_DM_W::new(self)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bck_out_dm(&mut self) -> RX_BCK_OUT_DM_W<RX_TIMING_SPEC, 20> {
        RX_BCK_OUT_DM_W::new(self)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ws_in_dm(&mut self) -> RX_WS_IN_DM_W<RX_TIMING_SPEC, 24> {
        RX_WS_IN_DM_W::new(self)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bck_in_dm(&mut self) -> RX_BCK_IN_DM_W<RX_TIMING_SPEC, 28> {
        RX_BCK_IN_DM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S RX timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_timing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_timing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_TIMING_SPEC;
impl crate::RegisterSpec for RX_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_timing::R`](R) reader structure"]
impl crate::Readable for RX_TIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_timing::W`](W) writer structure"]
impl crate::Writable for RX_TIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_TIMING to value 0"]
impl crate::Resettable for RX_TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
