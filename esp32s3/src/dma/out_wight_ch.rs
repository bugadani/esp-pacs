#[doc = "Register `OUT_WIGHT_CH%s` reader"]
pub type R = crate::R<OUT_WIGHT_CH_SPEC>;
#[doc = "Register `OUT_WIGHT_CH%s` writer"]
pub type W = crate::W<OUT_WIGHT_CH_SPEC>;
#[doc = "Field `TX_WEIGHT` reader - The weight of Tx channel 0."]
pub type TX_WEIGHT_R = crate::FieldReader;
#[doc = "Field `TX_WEIGHT` writer - The weight of Tx channel 0."]
pub type TX_WEIGHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight(&self) -> TX_WEIGHT_R {
        TX_WEIGHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_WIGHT_CH")
            .field("tx_weight", &format_args!("{}", self.tx_weight().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_WIGHT_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_weight(&mut self) -> TX_WEIGHT_W<OUT_WIGHT_CH_SPEC, 8> {
        TX_WEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Weight register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_wight_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_wight_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_WIGHT_CH_SPEC;
impl crate::RegisterSpec for OUT_WIGHT_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_wight_ch::R`](R) reader structure"]
impl crate::Readable for OUT_WIGHT_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_wight_ch::W`](W) writer structure"]
impl crate::Writable for OUT_WIGHT_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_WIGHT_CH%s to value 0x0f00"]
impl crate::Resettable for OUT_WIGHT_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
