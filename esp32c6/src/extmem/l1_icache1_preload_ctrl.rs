#[doc = "Register `L1_ICACHE1_PRELOAD_CTRL` reader"]
pub type R = crate::R<L1_ICACHE1_PRELOAD_CTRL_SPEC>;
#[doc = "Register `L1_ICACHE1_PRELOAD_CTRL` writer"]
pub type W = crate::W<L1_ICACHE1_PRELOAD_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE1_PRELOAD_ENA` reader - The bit is used to enable preload operation on L1-ICache1. It will be cleared by hardware automatically after preload operation is done."]
pub type L1_ICACHE1_PRELOAD_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PRELOAD_ENA` writer - The bit is used to enable preload operation on L1-ICache1. It will be cleared by hardware automatically after preload operation is done."]
pub type L1_ICACHE1_PRELOAD_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L1_ICACHE1_PRELOAD_DONE` reader - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
pub type L1_ICACHE1_PRELOAD_DONE_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
pub type L1_ICACHE1_PRELOAD_ORDER_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PRELOAD_RGID` reader - The bit is used to set the gid of l1 icache1 preload."]
pub type L1_ICACHE1_PRELOAD_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L1-ICache1. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    pub fn l1_icache1_preload_ena(&self) -> L1_ICACHE1_PRELOAD_ENA_R {
        L1_ICACHE1_PRELOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_icache1_preload_done(&self) -> L1_ICACHE1_PRELOAD_DONE_R {
        L1_ICACHE1_PRELOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
    #[inline(always)]
    pub fn l1_icache1_preload_order(&self) -> L1_ICACHE1_PRELOAD_ORDER_R {
        L1_ICACHE1_PRELOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of l1 icache1 preload."]
    #[inline(always)]
    pub fn l1_icache1_preload_rgid(&self) -> L1_ICACHE1_PRELOAD_RGID_R {
        L1_ICACHE1_PRELOAD_RGID_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE1_PRELOAD_CTRL")
            .field(
                "l1_icache1_preload_ena",
                &format_args!("{}", self.l1_icache1_preload_ena().bit()),
            )
            .field(
                "l1_icache1_preload_done",
                &format_args!("{}", self.l1_icache1_preload_done().bit()),
            )
            .field(
                "l1_icache1_preload_order",
                &format_args!("{}", self.l1_icache1_preload_order().bit()),
            )
            .field(
                "l1_icache1_preload_rgid",
                &format_args!("{}", self.l1_icache1_preload_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE1_PRELOAD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L1-ICache1. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_preload_ena(
        &mut self,
    ) -> L1_ICACHE1_PRELOAD_ENA_W<L1_ICACHE1_PRELOAD_CTRL_SPEC, 0> {
        L1_ICACHE1_PRELOAD_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "L1 instruction Cache 1 preload-operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_preload_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_preload_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_ICACHE1_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_ICACHE1_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache1_preload_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_ICACHE1_PRELOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_icache1_preload_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_ICACHE1_PRELOAD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_ICACHE1_PRELOAD_CTRL to value 0x02"]
impl crate::Resettable for L1_ICACHE1_PRELOAD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
