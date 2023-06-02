#[doc = "Register `ETM_TASK_P6_CFG` reader"]
pub struct R(crate::R<ETM_TASK_P6_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETM_TASK_P6_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETM_TASK_P6_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETM_TASK_P6_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETM_TASK_P6_CFG` writer"]
pub struct W(crate::W<ETM_TASK_P6_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETM_TASK_P6_CFG_SPEC>;
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
impl From<crate::W<ETM_TASK_P6_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETM_TASK_P6_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_TASK_GPIO24_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO24_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO24_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO24_EN_W<'a, const O: u8> = crate::BitWriter<'a, ETM_TASK_P6_CFG_SPEC, O>;
#[doc = "Field `ETM_TASK_GPIO24_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO24_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO24_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO24_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, ETM_TASK_P6_CFG_SPEC, 3, O>;
#[doc = "Field `ETM_TASK_GPIO25_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO25_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO25_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO25_EN_W<'a, const O: u8> = crate::BitWriter<'a, ETM_TASK_P6_CFG_SPEC, O>;
#[doc = "Field `ETM_TASK_GPIO25_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO25_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO25_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO25_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, ETM_TASK_P6_CFG_SPEC, 3, O>;
#[doc = "Field `ETM_TASK_GPIO26_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO26_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO26_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO26_EN_W<'a, const O: u8> = crate::BitWriter<'a, ETM_TASK_P6_CFG_SPEC, O>;
#[doc = "Field `ETM_TASK_GPIO26_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO26_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO26_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO26_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, ETM_TASK_P6_CFG_SPEC, 3, O>;
#[doc = "Field `ETM_TASK_GPIO27_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO27_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO27_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO27_EN_W<'a, const O: u8> = crate::BitWriter<'a, ETM_TASK_P6_CFG_SPEC, O>;
#[doc = "Field `ETM_TASK_GPIO27_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO27_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO27_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO27_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, ETM_TASK_P6_CFG_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio24_en(&self) -> ETM_TASK_GPIO24_EN_R {
        ETM_TASK_GPIO24_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio24_sel(&self) -> ETM_TASK_GPIO24_SEL_R {
        ETM_TASK_GPIO24_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio25_en(&self) -> ETM_TASK_GPIO25_EN_R {
        ETM_TASK_GPIO25_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio25_sel(&self) -> ETM_TASK_GPIO25_SEL_R {
        ETM_TASK_GPIO25_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio26_en(&self) -> ETM_TASK_GPIO26_EN_R {
        ETM_TASK_GPIO26_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio26_sel(&self) -> ETM_TASK_GPIO26_SEL_R {
        ETM_TASK_GPIO26_SEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio27_en(&self) -> ETM_TASK_GPIO27_EN_R {
        ETM_TASK_GPIO27_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio27_sel(&self) -> ETM_TASK_GPIO27_SEL_R {
        ETM_TASK_GPIO27_SEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P6_CFG")
            .field(
                "etm_task_gpio24_en",
                &format_args!("{}", self.etm_task_gpio24_en().bit()),
            )
            .field(
                "etm_task_gpio24_sel",
                &format_args!("{}", self.etm_task_gpio24_sel().bits()),
            )
            .field(
                "etm_task_gpio25_en",
                &format_args!("{}", self.etm_task_gpio25_en().bit()),
            )
            .field(
                "etm_task_gpio25_sel",
                &format_args!("{}", self.etm_task_gpio25_sel().bits()),
            )
            .field(
                "etm_task_gpio26_en",
                &format_args!("{}", self.etm_task_gpio26_en().bit()),
            )
            .field(
                "etm_task_gpio26_sel",
                &format_args!("{}", self.etm_task_gpio26_sel().bits()),
            )
            .field(
                "etm_task_gpio27_en",
                &format_args!("{}", self.etm_task_gpio27_en().bit()),
            )
            .field(
                "etm_task_gpio27_sel",
                &format_args!("{}", self.etm_task_gpio27_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_TASK_P6_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio24_en(&mut self) -> ETM_TASK_GPIO24_EN_W<0> {
        ETM_TASK_GPIO24_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio24_sel(&mut self) -> ETM_TASK_GPIO24_SEL_W<1> {
        ETM_TASK_GPIO24_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio25_en(&mut self) -> ETM_TASK_GPIO25_EN_W<8> {
        ETM_TASK_GPIO25_EN_W::new(self)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio25_sel(&mut self) -> ETM_TASK_GPIO25_SEL_W<9> {
        ETM_TASK_GPIO25_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio26_en(&mut self) -> ETM_TASK_GPIO26_EN_W<16> {
        ETM_TASK_GPIO26_EN_W::new(self)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio26_sel(&mut self) -> ETM_TASK_GPIO26_SEL_W<17> {
        ETM_TASK_GPIO26_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio27_en(&mut self) -> ETM_TASK_GPIO27_EN_W<24> {
        ETM_TASK_GPIO27_EN_W::new(self)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio27_sel(&mut self) -> ETM_TASK_GPIO27_SEL_W<25> {
        ETM_TASK_GPIO27_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etm_task_p6_cfg](index.html) module"]
pub struct ETM_TASK_P6_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P6_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etm_task_p6_cfg::R](R) reader structure"]
impl crate::Readable for ETM_TASK_P6_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etm_task_p6_cfg::W](W) writer structure"]
impl crate::Writable for ETM_TASK_P6_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_TASK_P6_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P6_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
