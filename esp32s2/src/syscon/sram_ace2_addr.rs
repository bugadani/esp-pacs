#[doc = "Register `SRAM_ACE2_ADDR` reader"]
pub type R = crate::R<SRAM_ACE2_ADDR_SPEC>;
#[doc = "Register `SRAM_ACE2_ADDR` writer"]
pub type W = crate::W<SRAM_ACE2_ADDR_SPEC>;
#[doc = "Field `S` reader - "]
pub type S_R = crate::FieldReader<u32>;
#[doc = "Field `S` writer - "]
pub type S_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE2_ADDR")
            .field("s", &format_args!("{}", self.s().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_ACE2_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<SRAM_ACE2_ADDR_SPEC, 0> {
        S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE2_ADDR_SPEC;
impl crate::RegisterSpec for SRAM_ACE2_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ace2_addr::R`](R) reader structure"]
impl crate::Readable for SRAM_ACE2_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ace2_addr::W`](W) writer structure"]
impl crate::Writable for SRAM_ACE2_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_ACE2_ADDR to value 0x2000_0000"]
impl crate::Resettable for SRAM_ACE2_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
