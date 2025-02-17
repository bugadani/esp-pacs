#[doc = "Register `BLE_TIMER_INT_MAP` reader"]
pub type R = crate::R<BLE_TIMER_INT_MAP_SPEC>;
#[doc = "Register `BLE_TIMER_INT_MAP` writer"]
pub type W = crate::W<BLE_TIMER_INT_MAP_SPEC>;
#[doc = "Field `BLE_TIMER_INT_MAP` reader - Need add description"]
pub type BLE_TIMER_INT_MAP_R = crate::FieldReader;
#[doc = "Field `BLE_TIMER_INT_MAP` writer - Need add description"]
pub type BLE_TIMER_INT_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn ble_timer_int_map(&self) -> BLE_TIMER_INT_MAP_R {
        BLE_TIMER_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_TIMER_INT_MAP")
            .field(
                "ble_timer_int_map",
                &format_args!("{}", self.ble_timer_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLE_TIMER_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn ble_timer_int_map(&mut self) -> BLE_TIMER_INT_MAP_W<BLE_TIMER_INT_MAP_SPEC, 0> {
        BLE_TIMER_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ble_timer_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_timer_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLE_TIMER_INT_MAP_SPEC;
impl crate::RegisterSpec for BLE_TIMER_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ble_timer_int_map::R`](R) reader structure"]
impl crate::Readable for BLE_TIMER_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ble_timer_int_map::W`](W) writer structure"]
impl crate::Writable for BLE_TIMER_INT_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLE_TIMER_INT_MAP to value 0"]
impl crate::Resettable for BLE_TIMER_INT_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
