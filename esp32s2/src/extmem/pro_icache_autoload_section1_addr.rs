#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_ADDR` reader"]
pub type R = crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_ADDR` writer"]
pub type W = crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ADDR` reader - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ADDR` writer - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_addr(&self) -> PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R {
        PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_AUTOLOAD_SECTION1_ADDR")
            .field(
                "pro_icache_autoload_sct1_addr",
                &format_args!("{}", self.pro_icache_autoload_sct1_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_sct1_addr(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC, 0> {
        PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_icache_autoload_section1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_icache_autoload_section1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_icache_autoload_section1_addr::R`](R) reader structure"]
impl crate::Readable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_icache_autoload_section1_addr::W`](W) writer structure"]
impl crate::Writable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_ICACHE_AUTOLOAD_SECTION1_ADDR to value 0"]
impl crate::Resettable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
