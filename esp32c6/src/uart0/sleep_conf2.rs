#[doc = "Register `SLEEP_CONF2` reader"]
pub type R = crate::R<SLEEP_CONF2_SPEC>;
#[doc = "Register `SLEEP_CONF2` writer"]
pub type W = crate::W<SLEEP_CONF2_SPEC>;
#[doc = "Field `ACTIVE_THRESHOLD` reader - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
pub type ACTIVE_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `ACTIVE_THRESHOLD` writer - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
pub type ACTIVE_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `RX_WAKE_UP_THRHD` reader - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
pub type RX_WAKE_UP_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_WAKE_UP_THRHD` writer - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
pub type RX_WAKE_UP_THRHD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WK_CHAR_NUM` reader - This register is used to select number of wake up char."]
pub type WK_CHAR_NUM_R = crate::FieldReader;
#[doc = "Field `WK_CHAR_NUM` writer - This register is used to select number of wake up char."]
pub type WK_CHAR_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `WK_CHAR_MASK` reader - This register is used to mask wake up char."]
pub type WK_CHAR_MASK_R = crate::FieldReader;
#[doc = "Field `WK_CHAR_MASK` writer - This register is used to mask wake up char."]
pub type WK_CHAR_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `WK_MODE_SEL` reader - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
pub type WK_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `WK_MODE_SEL` writer - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
pub type WK_MODE_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:9 - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
    #[inline(always)]
    pub fn active_threshold(&self) -> ACTIVE_THRESHOLD_R {
        ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:17 - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
    #[inline(always)]
    pub fn rx_wake_up_thrhd(&self) -> RX_WAKE_UP_THRHD_R {
        RX_WAKE_UP_THRHD_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:20 - This register is used to select number of wake up char."]
    #[inline(always)]
    pub fn wk_char_num(&self) -> WK_CHAR_NUM_R {
        WK_CHAR_NUM_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:25 - This register is used to mask wake up char."]
    #[inline(always)]
    pub fn wk_char_mask(&self) -> WK_CHAR_MASK_R {
        WK_CHAR_MASK_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
    #[inline(always)]
    pub fn wk_mode_sel(&self) -> WK_MODE_SEL_R {
        WK_MODE_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF2")
            .field(
                "active_threshold",
                &format_args!("{}", self.active_threshold().bits()),
            )
            .field(
                "rx_wake_up_thrhd",
                &format_args!("{}", self.rx_wake_up_thrhd().bits()),
            )
            .field(
                "wk_char_num",
                &format_args!("{}", self.wk_char_num().bits()),
            )
            .field(
                "wk_char_mask",
                &format_args!("{}", self.wk_char_mask().bits()),
            )
            .field(
                "wk_mode_sel",
                &format_args!("{}", self.wk_mode_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLEEP_CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
    #[inline(always)]
    #[must_use]
    pub fn active_threshold(&mut self) -> ACTIVE_THRESHOLD_W<SLEEP_CONF2_SPEC, 0> {
        ACTIVE_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 10:17 - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wake_up_thrhd(&mut self) -> RX_WAKE_UP_THRHD_W<SLEEP_CONF2_SPEC, 10> {
        RX_WAKE_UP_THRHD_W::new(self)
    }
    #[doc = "Bits 18:20 - This register is used to select number of wake up char."]
    #[inline(always)]
    #[must_use]
    pub fn wk_char_num(&mut self) -> WK_CHAR_NUM_W<SLEEP_CONF2_SPEC, 18> {
        WK_CHAR_NUM_W::new(self)
    }
    #[doc = "Bits 21:25 - This register is used to mask wake up char."]
    #[inline(always)]
    #[must_use]
    pub fn wk_char_mask(&mut self) -> WK_CHAR_MASK_W<SLEEP_CONF2_SPEC, 21> {
        WK_CHAR_MASK_W::new(self)
    }
    #[doc = "Bits 26:27 - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
    #[inline(always)]
    #[must_use]
    pub fn wk_mode_sel(&mut self) -> WK_MODE_SEL_W<SLEEP_CONF2_SPEC, 26> {
        WK_MODE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART sleep configure register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF2_SPEC;
impl crate::RegisterSpec for SLEEP_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf2::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf2::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEP_CONF2 to value 0x0014_04f0"]
impl crate::Resettable for SLEEP_CONF2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0014_04f0;
}
