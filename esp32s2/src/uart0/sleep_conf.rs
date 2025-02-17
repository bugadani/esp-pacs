#[doc = "Register `SLEEP_CONF` reader"]
pub type R = crate::R<SLEEP_CONF_SPEC>;
#[doc = "Register `SLEEP_CONF` writer"]
pub type W = crate::W<SLEEP_CONF_SPEC>;
#[doc = "Field `ACTIVE_THRESHOLD` reader - The UART is activated from Light-sleep mode when the input RXD edge changes more times than this register's value."]
pub type ACTIVE_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `ACTIVE_THRESHOLD` writer - The UART is activated from Light-sleep mode when the input RXD edge changes more times than this register's value."]
pub type ACTIVE_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - The UART is activated from Light-sleep mode when the input RXD edge changes more times than this register's value."]
    #[inline(always)]
    pub fn active_threshold(&self) -> ACTIVE_THRESHOLD_R {
        ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF")
            .field(
                "active_threshold",
                &format_args!("{}", self.active_threshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLEEP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - The UART is activated from Light-sleep mode when the input RXD edge changes more times than this register's value."]
    #[inline(always)]
    #[must_use]
    pub fn active_threshold(&mut self) -> ACTIVE_THRESHOLD_W<SLEEP_CONF_SPEC, 0> {
        ACTIVE_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sleep mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF_SPEC;
impl crate::RegisterSpec for SLEEP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEP_CONF to value 0xf0"]
impl crate::Resettable for SLEEP_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
