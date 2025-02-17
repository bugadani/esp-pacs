#[doc = "Register `DATA_1` reader"]
pub type R = crate::R<DATA_1_SPEC>;
#[doc = "Register `DATA_1` writer"]
pub type W = crate::W<DATA_1_SPEC>;
#[doc = "Field `TX_BYTE_1` reader - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1."]
pub type TX_BYTE_1_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_1` writer - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1."]
pub type TX_BYTE_1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1."]
    #[inline(always)]
    pub fn tx_byte_1(&self) -> TX_BYTE_1_R {
        TX_BYTE_1_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_1")
            .field("tx_byte_1", &format_args!("{}", self.tx_byte_1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 1 and when software initiate read operation, it is rx data register 1."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_1(&mut self) -> TX_BYTE_1_W<DATA_1_SPEC, 0> {
        TX_BYTE_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_1_SPEC;
impl crate::RegisterSpec for DATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_1::R`](R) reader structure"]
impl crate::Readable for DATA_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_1::W`](W) writer structure"]
impl crate::Writable for DATA_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_1 to value 0"]
impl crate::Resettable for DATA_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
