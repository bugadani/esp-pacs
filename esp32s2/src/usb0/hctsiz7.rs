#[doc = "Register `HCTSIZ7` reader"]
pub type R = crate::R<HCTSIZ7_SPEC>;
#[doc = "Register `HCTSIZ7` writer"]
pub type W = crate::W<HCTSIZ7_SPEC>;
#[doc = "Field `H_XFERSIZE7` reader - "]
pub type H_XFERSIZE7_R = crate::FieldReader<u32>;
#[doc = "Field `H_XFERSIZE7` writer - "]
pub type H_XFERSIZE7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 19, O, u32>;
#[doc = "Field `H_PKTCNT7` reader - "]
pub type H_PKTCNT7_R = crate::FieldReader<u16>;
#[doc = "Field `H_PKTCNT7` writer - "]
pub type H_PKTCNT7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `H_PID7` reader - "]
pub type H_PID7_R = crate::FieldReader;
#[doc = "Field `H_PID7` writer - "]
pub type H_PID7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `H_DOPNG7` reader - "]
pub type H_DOPNG7_R = crate::BitReader;
#[doc = "Field `H_DOPNG7` writer - "]
pub type H_DOPNG7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize7(&self) -> H_XFERSIZE7_R {
        H_XFERSIZE7_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt7(&self) -> H_PKTCNT7_R {
        H_PKTCNT7_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid7(&self) -> H_PID7_R {
        H_PID7_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng7(&self) -> H_DOPNG7_R {
        H_DOPNG7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ7")
            .field(
                "h_xfersize7",
                &format_args!("{}", self.h_xfersize7().bits()),
            )
            .field("h_pktcnt7", &format_args!("{}", self.h_pktcnt7().bits()))
            .field("h_pid7", &format_args!("{}", self.h_pid7().bits()))
            .field("h_dopng7", &format_args!("{}", self.h_dopng7().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCTSIZ7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfersize7(&mut self) -> H_XFERSIZE7_W<HCTSIZ7_SPEC, 0> {
        H_XFERSIZE7_W::new(self)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_pktcnt7(&mut self) -> H_PKTCNT7_W<HCTSIZ7_SPEC, 19> {
        H_PKTCNT7_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn h_pid7(&mut self) -> H_PID7_W<HCTSIZ7_SPEC, 29> {
        H_PID7_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dopng7(&mut self) -> H_DOPNG7_W<HCTSIZ7_SPEC, 31> {
        H_DOPNG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ7_SPEC;
impl crate::RegisterSpec for HCTSIZ7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz7::R`](R) reader structure"]
impl crate::Readable for HCTSIZ7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz7::W`](W) writer structure"]
impl crate::Writable for HCTSIZ7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTSIZ7 to value 0"]
impl crate::Resettable for HCTSIZ7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
