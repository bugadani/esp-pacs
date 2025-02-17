#[doc = "Register `IN_SUC_EOF_DES_ADDR_CH%s` reader"]
pub type R = crate::R<IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Field `IN_SUC_EOF_DES_ADDR` reader - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
pub type IN_SUC_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr(&self) -> IN_SUC_EOF_DES_ADDR_R {
        IN_SUC_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_SUC_EOF_DES_ADDR_CH")
            .field(
                "in_suc_eof_des_addr",
                &format_args!("{}", self.in_suc_eof_des_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_SUC_EOF_DES_ADDR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Inlink descriptor address when EOF occurs of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SUC_EOF_DES_ADDR_CH_SPEC;
impl crate::RegisterSpec for IN_SUC_EOF_DES_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_suc_eof_des_addr_ch::R`](R) reader structure"]
impl crate::Readable for IN_SUC_EOF_DES_ADDR_CH_SPEC {}
#[doc = "`reset()` method sets IN_SUC_EOF_DES_ADDR_CH%s to value 0"]
impl crate::Resettable for IN_SUC_EOF_DES_ADDR_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
