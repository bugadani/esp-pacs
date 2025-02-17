#[doc = "Register `OUT_PRI_CH%s` reader"]
pub type R = crate::R<OUT_PRI_CH_SPEC>;
#[doc = "Register `OUT_PRI_CH%s` writer"]
pub type W = crate::W<OUT_PRI_CH_SPEC>;
#[doc = "Field `TX_PRI` reader - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
pub type TX_PRI_R = crate::FieldReader;
#[doc = "Field `TX_PRI` writer - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
pub type TX_PRI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri(&self) -> TX_PRI_R {
        TX_PRI_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PRI_CH")
            .field("tx_pri", &format_args!("{}", self.tx_pri().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_PRI_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pri(&mut self) -> TX_PRI_W<OUT_PRI_CH_SPEC, 0> {
        TX_PRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Priority register of Tx channel 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PRI_CH_SPEC;
impl crate::RegisterSpec for OUT_PRI_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_pri_ch::R`](R) reader structure"]
impl crate::Readable for OUT_PRI_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_pri_ch::W`](W) writer structure"]
impl crate::Writable for OUT_PRI_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_PRI_CH%s to value 0"]
impl crate::Resettable for OUT_PRI_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
