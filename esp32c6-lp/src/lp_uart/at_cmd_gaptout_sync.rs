#[doc = "Register `AT_CMD_GAPTOUT_SYNC` reader"]
pub type R = crate::R<AT_CMD_GAPTOUT_SYNC_SPEC>;
#[doc = "Register `AT_CMD_GAPTOUT_SYNC` writer"]
pub type W = crate::W<AT_CMD_GAPTOUT_SYNC_SPEC>;
#[doc = "Field `RX_GAP_TOUT` reader - This register is used to configure the duration time between the at_cmd chars."]
pub type RX_GAP_TOUT_R = crate::FieldReader<u16>;
#[doc = "Field `RX_GAP_TOUT` writer - This register is used to configure the duration time between the at_cmd chars."]
pub type RX_GAP_TOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the at_cmd chars."]
    #[inline(always)]
    pub fn rx_gap_tout(&self) -> RX_GAP_TOUT_R {
        RX_GAP_TOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_GAPTOUT_SYNC")
            .field(
                "rx_gap_tout",
                &format_args!("{}", self.rx_gap_tout().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AT_CMD_GAPTOUT_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the at_cmd chars."]
    #[inline(always)]
    #[must_use]
    pub fn rx_gap_tout(&mut self) -> RX_GAP_TOUT_W<AT_CMD_GAPTOUT_SYNC_SPEC, 0> {
        RX_GAP_TOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timeout configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_gaptout_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_gaptout_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AT_CMD_GAPTOUT_SYNC_SPEC;
impl crate::RegisterSpec for AT_CMD_GAPTOUT_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_gaptout_sync::R`](R) reader structure"]
impl crate::Readable for AT_CMD_GAPTOUT_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_gaptout_sync::W`](W) writer structure"]
impl crate::Writable for AT_CMD_GAPTOUT_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AT_CMD_GAPTOUT_SYNC to value 0x0b"]
impl crate::Resettable for AT_CMD_GAPTOUT_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
