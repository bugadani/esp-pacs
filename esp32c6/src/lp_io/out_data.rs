#[doc = "Register `OUT_DATA` reader"]
pub type R = crate::R<OUT_DATA_SPEC>;
#[doc = "Register `OUT_DATA` writer"]
pub type W = crate::W<OUT_DATA_SPEC>;
#[doc = "Field `LP_GPIO_OUT_DATA` reader - set lp gpio output data"]
pub type LP_GPIO_OUT_DATA_R = crate::FieldReader;
#[doc = "Field `LP_GPIO_OUT_DATA` writer - set lp gpio output data"]
pub type LP_GPIO_OUT_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - set lp gpio output data"]
    #[inline(always)]
    pub fn lp_gpio_out_data(&self) -> LP_GPIO_OUT_DATA_R {
        LP_GPIO_OUT_DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DATA")
            .field(
                "lp_gpio_out_data",
                &format_args!("{}", self.lp_gpio_out_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - set lp gpio output data"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio_out_data(&mut self) -> LP_GPIO_OUT_DATA_W<OUT_DATA_SPEC, 0> {
        LP_GPIO_OUT_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_DATA_SPEC;
impl crate::RegisterSpec for OUT_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_data::R`](R) reader structure"]
impl crate::Readable for OUT_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_data::W`](W) writer structure"]
impl crate::Writable for OUT_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_DATA to value 0"]
impl crate::Resettable for OUT_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
