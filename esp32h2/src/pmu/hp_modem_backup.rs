#[doc = "Register `HP_MODEM_BACKUP` reader"]
pub type R = crate::R<HP_MODEM_BACKUP_SPEC>;
#[doc = "Register `HP_MODEM_BACKUP` writer"]
pub type W = crate::W<HP_MODEM_BACKUP_SPEC>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE` reader - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HP_MODEM_RETENTION_MODE` reader - need_des"]
pub type HP_MODEM_RETENTION_MODE_R = crate::BitReader;
#[doc = "Field `HP_MODEM_RETENTION_MODE` writer - need_des"]
pub type HP_MODEM_RETENTION_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HP_SLEEP2MODEM_RETENTION_EN` reader - need_des"]
pub type HP_SLEEP2MODEM_RETENTION_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP2MODEM_RETENTION_EN` writer - need_des"]
pub type HP_SLEEP2MODEM_RETENTION_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_CLK_SEL` reader - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_CLK_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODE` reader - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODE` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_EN` reader - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_EN` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_modem_clk_code(&self) -> HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_R {
        HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn hp_modem_retention_mode(&self) -> HP_MODEM_RETENTION_MODE_R {
        HP_MODEM_RETENTION_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_retention_en(&self) -> HP_SLEEP2MODEM_RETENTION_EN_R {
        HP_SLEEP2MODEM_RETENTION_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_clk_sel(&self) -> HP_SLEEP2MODEM_BACKUP_CLK_SEL_R {
        HP_SLEEP2MODEM_BACKUP_CLK_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 20:22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_mode(&self) -> HP_SLEEP2MODEM_BACKUP_MODE_R {
        HP_SLEEP2MODEM_BACKUP_MODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_en(&self) -> HP_SLEEP2MODEM_BACKUP_EN_R {
        HP_SLEEP2MODEM_BACKUP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_BACKUP")
            .field(
                "hp_sleep2modem_backup_modem_clk_code",
                &format_args!("{}", self.hp_sleep2modem_backup_modem_clk_code().bits()),
            )
            .field(
                "hp_modem_retention_mode",
                &format_args!("{}", self.hp_modem_retention_mode().bit()),
            )
            .field(
                "hp_sleep2modem_retention_en",
                &format_args!("{}", self.hp_sleep2modem_retention_en().bit()),
            )
            .field(
                "hp_sleep2modem_backup_clk_sel",
                &format_args!("{}", self.hp_sleep2modem_backup_clk_sel().bits()),
            )
            .field(
                "hp_sleep2modem_backup_mode",
                &format_args!("{}", self.hp_sleep2modem_backup_mode().bits()),
            )
            .field(
                "hp_sleep2modem_backup_en",
                &format_args!("{}", self.hp_sleep2modem_backup_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_BACKUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_modem_clk_code(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W<HP_MODEM_BACKUP_SPEC, 4> {
        HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W::new(self)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_retention_mode(
        &mut self,
    ) -> HP_MODEM_RETENTION_MODE_W<HP_MODEM_BACKUP_SPEC, 10> {
        HP_MODEM_RETENTION_MODE_W::new(self)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_retention_en(
        &mut self,
    ) -> HP_SLEEP2MODEM_RETENTION_EN_W<HP_MODEM_BACKUP_SPEC, 11> {
        HP_SLEEP2MODEM_RETENTION_EN_W::new(self)
    }
    #[doc = "Bits 14:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_clk_sel(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_CLK_SEL_W<HP_MODEM_BACKUP_SPEC, 14> {
        HP_SLEEP2MODEM_BACKUP_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 20:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_mode(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_MODE_W<HP_MODEM_BACKUP_SPEC, 20> {
        HP_SLEEP2MODEM_BACKUP_MODE_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_en(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_EN_W<HP_MODEM_BACKUP_SPEC, 29> {
        HP_SLEEP2MODEM_BACKUP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_modem_backup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_backup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_BACKUP_SPEC;
impl crate::RegisterSpec for HP_MODEM_BACKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_backup::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_BACKUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_backup::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_BACKUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_BACKUP to value 0"]
impl crate::Resettable for HP_MODEM_BACKUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
