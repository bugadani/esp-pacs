#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TIMER0_STOP_INT_CLR` writer - "]
pub type TIMER0_STOP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1_STOP_INT_CLR` writer - "]
pub type TIMER1_STOP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER2_STOP_INT_CLR` writer - "]
pub type TIMER2_STOP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER0_TEZ_INT_CLR` writer - "]
pub type TIMER0_TEZ_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1_TEZ_INT_CLR` writer - "]
pub type TIMER1_TEZ_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER2_TEZ_INT_CLR` writer - "]
pub type TIMER2_TEZ_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER0_TEP_INT_CLR` writer - "]
pub type TIMER0_TEP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1_TEP_INT_CLR` writer - "]
pub type TIMER1_TEP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER2_TEP_INT_CLR` writer - "]
pub type TIMER2_TEP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT0_INT_CLR` writer - "]
pub type FAULT0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT1_INT_CLR` writer - "]
pub type FAULT1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT2_INT_CLR` writer - "]
pub type FAULT2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT0_CLR_INT_CLR` writer - "]
pub type FAULT0_CLR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT1_CLR_INT_CLR` writer - "]
pub type FAULT1_CLR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT2_CLR_INT_CLR` writer - "]
pub type FAULT2_CLR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OP0_TEA_INT_CLR` writer - "]
pub type OP0_TEA_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OP1_TEA_INT_CLR` writer - "]
pub type OP1_TEA_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OP2_TEA_INT_CLR` writer - "]
pub type OP2_TEA_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OP0_TEB_INT_CLR` writer - "]
pub type OP0_TEB_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OP1_TEB_INT_CLR` writer - "]
pub type OP1_TEB_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OP2_TEB_INT_CLR` writer - "]
pub type OP2_TEB_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH0_CBC_INT_CLR` writer - "]
pub type FH0_CBC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH1_CBC_INT_CLR` writer - "]
pub type FH1_CBC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH2_CBC_INT_CLR` writer - "]
pub type FH2_CBC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH0_OST_INT_CLR` writer - "]
pub type FH0_OST_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH1_OST_INT_CLR` writer - "]
pub type FH1_OST_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH2_OST_INT_CLR` writer - "]
pub type FH2_OST_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0_INT_CLR` writer - "]
pub type CAP0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1_INT_CLR` writer - "]
pub type CAP1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP2_INT_CLR` writer - "]
pub type CAP2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_stop_int_clr(&mut self) -> TIMER0_STOP_INT_CLR_W<INT_CLR_SPEC, 0> {
        TIMER0_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_stop_int_clr(&mut self) -> TIMER1_STOP_INT_CLR_W<INT_CLR_SPEC, 1> {
        TIMER1_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_stop_int_clr(&mut self) -> TIMER2_STOP_INT_CLR_W<INT_CLR_SPEC, 2> {
        TIMER2_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tez_int_clr(&mut self) -> TIMER0_TEZ_INT_CLR_W<INT_CLR_SPEC, 3> {
        TIMER0_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tez_int_clr(&mut self) -> TIMER1_TEZ_INT_CLR_W<INT_CLR_SPEC, 4> {
        TIMER1_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tez_int_clr(&mut self) -> TIMER2_TEZ_INT_CLR_W<INT_CLR_SPEC, 5> {
        TIMER2_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tep_int_clr(&mut self) -> TIMER0_TEP_INT_CLR_W<INT_CLR_SPEC, 6> {
        TIMER0_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tep_int_clr(&mut self) -> TIMER1_TEP_INT_CLR_W<INT_CLR_SPEC, 7> {
        TIMER1_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tep_int_clr(&mut self) -> TIMER2_TEP_INT_CLR_W<INT_CLR_SPEC, 8> {
        TIMER2_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn fault0_int_clr(&mut self) -> FAULT0_INT_CLR_W<INT_CLR_SPEC, 9> {
        FAULT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn fault1_int_clr(&mut self) -> FAULT1_INT_CLR_W<INT_CLR_SPEC, 10> {
        FAULT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn fault2_int_clr(&mut self) -> FAULT2_INT_CLR_W<INT_CLR_SPEC, 11> {
        FAULT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn fault0_clr_int_clr(&mut self) -> FAULT0_CLR_INT_CLR_W<INT_CLR_SPEC, 12> {
        FAULT0_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn fault1_clr_int_clr(&mut self) -> FAULT1_CLR_INT_CLR_W<INT_CLR_SPEC, 13> {
        FAULT1_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn fault2_clr_int_clr(&mut self) -> FAULT2_CLR_INT_CLR_W<INT_CLR_SPEC, 14> {
        FAULT2_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn op0_tea_int_clr(&mut self) -> OP0_TEA_INT_CLR_W<INT_CLR_SPEC, 15> {
        OP0_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn op1_tea_int_clr(&mut self) -> OP1_TEA_INT_CLR_W<INT_CLR_SPEC, 16> {
        OP1_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn op2_tea_int_clr(&mut self) -> OP2_TEA_INT_CLR_W<INT_CLR_SPEC, 17> {
        OP2_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn op0_teb_int_clr(&mut self) -> OP0_TEB_INT_CLR_W<INT_CLR_SPEC, 18> {
        OP0_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn op1_teb_int_clr(&mut self) -> OP1_TEB_INT_CLR_W<INT_CLR_SPEC, 19> {
        OP1_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn op2_teb_int_clr(&mut self) -> OP2_TEB_INT_CLR_W<INT_CLR_SPEC, 20> {
        OP2_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_cbc_int_clr(&mut self) -> FH0_CBC_INT_CLR_W<INT_CLR_SPEC, 21> {
        FH0_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_cbc_int_clr(&mut self) -> FH1_CBC_INT_CLR_W<INT_CLR_SPEC, 22> {
        FH1_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_cbc_int_clr(&mut self) -> FH2_CBC_INT_CLR_W<INT_CLR_SPEC, 23> {
        FH2_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_ost_int_clr(&mut self) -> FH0_OST_INT_CLR_W<INT_CLR_SPEC, 24> {
        FH0_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_ost_int_clr(&mut self) -> FH1_OST_INT_CLR_W<INT_CLR_SPEC, 25> {
        FH1_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_ost_int_clr(&mut self) -> FH2_OST_INT_CLR_W<INT_CLR_SPEC, 26> {
        FH2_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_int_clr(&mut self) -> CAP0_INT_CLR_W<INT_CLR_SPEC, 27> {
        CAP0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cap1_int_clr(&mut self) -> CAP1_INT_CLR_W<INT_CLR_SPEC, 28> {
        CAP1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_int_clr(&mut self) -> CAP2_INT_CLR_W<INT_CLR_SPEC, 29> {
        CAP2_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
