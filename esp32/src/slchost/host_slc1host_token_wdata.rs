#[doc = "Register `HOST_SLC1HOST_TOKEN_WDATA` reader"]
pub type R = crate::R<HOST_SLC1HOST_TOKEN_WDATA_SPEC>;
#[doc = "Register `HOST_SLC1HOST_TOKEN_WDATA` writer"]
pub type W = crate::W<HOST_SLC1HOST_TOKEN_WDATA_SPEC>;
#[doc = "Field `HOST_SLC1HOST_TOKEN0_WD` reader - "]
pub type HOST_SLC1HOST_TOKEN0_WD_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLC1HOST_TOKEN0_WD` writer - "]
pub type HOST_SLC1HOST_TOKEN0_WD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `HOST_SLC1HOST_TOKEN1_WD` reader - "]
pub type HOST_SLC1HOST_TOKEN1_WD_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLC1HOST_TOKEN1_WD` writer - "]
pub type HOST_SLC1HOST_TOKEN1_WD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc1host_token0_wd(&self) -> HOST_SLC1HOST_TOKEN0_WD_R {
        HOST_SLC1HOST_TOKEN0_WD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_slc1host_token1_wd(&self) -> HOST_SLC1HOST_TOKEN1_WD_R {
        HOST_SLC1HOST_TOKEN1_WD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC1HOST_TOKEN_WDATA")
            .field(
                "host_slc1host_token0_wd",
                &format_args!("{}", self.host_slc1host_token0_wd().bits()),
            )
            .field(
                "host_slc1host_token1_wd",
                &format_args!("{}", self.host_slc1host_token1_wd().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC1HOST_TOKEN_WDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token0_wd(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN0_WD_W<HOST_SLC1HOST_TOKEN_WDATA_SPEC, 0> {
        HOST_SLC1HOST_TOKEN0_WD_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token1_wd(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN1_WD_W<HOST_SLC1HOST_TOKEN_WDATA_SPEC, 16> {
        HOST_SLC1HOST_TOKEN1_WD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_token_wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_token_wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC1HOST_TOKEN_WDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC1HOST_TOKEN_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slc1host_token_wdata::R`](R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_TOKEN_WDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slc1host_token_wdata::W`](W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_TOKEN_WDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLC1HOST_TOKEN_WDATA to value 0"]
impl crate::Resettable for HOST_SLC1HOST_TOKEN_WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
