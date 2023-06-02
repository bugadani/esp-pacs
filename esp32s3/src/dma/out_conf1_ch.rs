#[doc = "Register `OUT_CONF1_CH%s` reader"]
pub struct R(crate::R<OUT_CONF1_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONF1_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONF1_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONF1_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONF1_CH%s` writer"]
pub struct W(crate::W<OUT_CONF1_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONF1_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OUT_CONF1_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONF1_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CONF1_CH_SPEC, O>;
#[doc = "Field `OUT_EXT_MEM_BK_SIZE` reader - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type OUT_EXT_MEM_BK_SIZE_R = crate::FieldReader;
#[doc = "Field `OUT_EXT_MEM_BK_SIZE` writer - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type OUT_EXT_MEM_BK_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_CONF1_CH_SPEC, 2, O>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&self) -> OUT_CHECK_OWNER_R {
        OUT_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn out_ext_mem_bk_size(&self) -> OUT_EXT_MEM_BK_SIZE_R {
        OUT_EXT_MEM_BK_SIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF1_CH")
            .field(
                "out_check_owner",
                &format_args!("{}", self.out_check_owner().bit()),
            )
            .field(
                "out_ext_mem_bk_size",
                &format_args!("{}", self.out_ext_mem_bk_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CONF1_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn out_check_owner(&mut self) -> OUT_CHECK_OWNER_W<12> {
        OUT_CHECK_OWNER_W::new(self)
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    #[must_use]
    pub fn out_ext_mem_bk_size(&mut self) -> OUT_EXT_MEM_BK_SIZE_W<13> {
        OUT_EXT_MEM_BK_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure 1 register of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf1_ch](index.html) module"]
pub struct OUT_CONF1_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf1_ch::R](R) reader structure"]
impl crate::Readable for OUT_CONF1_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf1_ch::W](W) writer structure"]
impl crate::Writable for OUT_CONF1_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CONF1_CH%s to value 0"]
impl crate::Resettable for OUT_CONF1_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
