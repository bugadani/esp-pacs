#[doc = "Register `FLASH_ACE0_SIZE` reader"]
pub type R = crate::R<FLASH_ACE0_SIZE_SPEC>;
#[doc = "Register `FLASH_ACE0_SIZE` writer"]
pub type W = crate::W<FLASH_ACE0_SIZE_SPEC>;
#[doc = "Field `FLASH_ACE0_SIZE` reader - reg_flash_ace0_size"]
pub type FLASH_ACE0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_ACE0_SIZE` writer - reg_flash_ace0_size"]
pub type FLASH_ACE0_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:12 - reg_flash_ace0_size"]
    #[inline(always)]
    pub fn flash_ace0_size(&self) -> FLASH_ACE0_SIZE_R {
        FLASH_ACE0_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE0_SIZE")
            .field(
                "flash_ace0_size",
                &format_args!("{}", self.flash_ace0_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASH_ACE0_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:12 - reg_flash_ace0_size"]
    #[inline(always)]
    #[must_use]
    pub fn flash_ace0_size(&mut self) -> FLASH_ACE0_SIZE_W<FLASH_ACE0_SIZE_SPEC, 0> {
        FLASH_ACE0_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB_CTRL_FLASH_ACE0_SIZE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE0_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_ACE0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ace0_size::R`](R) reader structure"]
impl crate::Readable for FLASH_ACE0_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ace0_size::W`](W) writer structure"]
impl crate::Writable for FLASH_ACE0_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_ACE0_SIZE to value 0x0400"]
impl crate::Resettable for FLASH_ACE0_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
