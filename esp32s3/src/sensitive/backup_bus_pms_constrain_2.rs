#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` reader - BackUp access bt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` writer - BackUp access bt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` reader - BackUp access i2c_ext0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` writer - BackUp access i2c_ext0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` reader - BackUp access uhci0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` writer - BackUp access uhci0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` reader - BackUp access slchost permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` writer - BackUp access slchost permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` reader - BackUp access rmt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` writer - BackUp access rmt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` reader - BackUp access pcnt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PCNT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` writer - BackUp access pcnt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` reader - BackUp access slc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` writer - BackUp access slc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` reader - BackUp access ledc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` writer - BackUp access ledc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` reader - BackUp access backup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` writer - BackUp access backup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` reader - BackUp access bb permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` writer - BackUp access bb permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` reader - BackUp access pwm0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM0_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` writer - BackUp access pwm0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` reader - BackUp access timergroup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` writer - BackUp access timergroup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` reader - BackUp access timergroup1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` writer - BackUp access timergroup1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` reader - BackUp access systimer permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` writer - BackUp access systimer permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - BackUp access bt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - BackUp access i2c_ext0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BackUp access uhci0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uhci0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - BackUp access slchost permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slchost(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BackUp access rmt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rmt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BackUp access pcnt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pcnt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_R {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BackUp access slc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - BackUp access ledc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ledc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_R {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - BackUp access backup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_backup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - BackUp access bb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bb(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BB_R {
        BACKUP_BUS_PMS_CONSTRAIN_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - BackUp access pwm0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwm0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - BackUp access timergroup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BackUp access timergroup1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BackUp access systimer permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_systimer(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_2")
            .field(
                "backup_bus_pms_constrain_bt",
                &format_args!("{}", self.backup_bus_pms_constrain_bt().bits()),
            )
            .field(
                "backup_bus_pms_constrain_i2c_ext0",
                &format_args!("{}", self.backup_bus_pms_constrain_i2c_ext0().bits()),
            )
            .field(
                "backup_bus_pms_constrain_uhci0",
                &format_args!("{}", self.backup_bus_pms_constrain_uhci0().bits()),
            )
            .field(
                "backup_bus_pms_constrain_slchost",
                &format_args!("{}", self.backup_bus_pms_constrain_slchost().bits()),
            )
            .field(
                "backup_bus_pms_constrain_rmt",
                &format_args!("{}", self.backup_bus_pms_constrain_rmt().bits()),
            )
            .field(
                "backup_bus_pms_constrain_pcnt",
                &format_args!("{}", self.backup_bus_pms_constrain_pcnt().bits()),
            )
            .field(
                "backup_bus_pms_constrain_slc",
                &format_args!("{}", self.backup_bus_pms_constrain_slc().bits()),
            )
            .field(
                "backup_bus_pms_constrain_ledc",
                &format_args!("{}", self.backup_bus_pms_constrain_ledc().bits()),
            )
            .field(
                "backup_bus_pms_constrain_backup",
                &format_args!("{}", self.backup_bus_pms_constrain_backup().bits()),
            )
            .field(
                "backup_bus_pms_constrain_bb",
                &format_args!("{}", self.backup_bus_pms_constrain_bb().bits()),
            )
            .field(
                "backup_bus_pms_constrain_pwm0",
                &format_args!("{}", self.backup_bus_pms_constrain_pwm0().bits()),
            )
            .field(
                "backup_bus_pms_constrain_timergroup",
                &format_args!("{}", self.backup_bus_pms_constrain_timergroup().bits()),
            )
            .field(
                "backup_bus_pms_constrain_timergroup1",
                &format_args!("{}", self.backup_bus_pms_constrain_timergroup1().bits()),
            )
            .field(
                "backup_bus_pms_constrain_systimer",
                &format_args!("{}", self.backup_bus_pms_constrain_systimer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - BackUp access bt permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 0> {
        BACKUP_BUS_PMS_CONSTRAIN_BT_W::new(self)
    }
    #[doc = "Bits 4:5 - BackUp access i2c_ext0 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_i2c_ext0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 4> {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W::new(self)
    }
    #[doc = "Bits 6:7 - BackUp access uhci0 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_uhci0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 6> {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W::new(self)
    }
    #[doc = "Bits 8:9 - BackUp access slchost permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_slchost(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 8> {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W::new(self)
    }
    #[doc = "Bits 10:11 - BackUp access rmt permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rmt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 10> {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_W::new(self)
    }
    #[doc = "Bits 12:13 - BackUp access pcnt permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_pcnt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 12> {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_W::new(self)
    }
    #[doc = "Bits 14:15 - BackUp access slc permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_slc(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 14> {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_W::new(self)
    }
    #[doc = "Bits 16:17 - BackUp access ledc permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_ledc(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 16> {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_W::new(self)
    }
    #[doc = "Bits 18:19 - BackUp access backup permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_backup(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 18> {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W::new(self)
    }
    #[doc = "Bits 22:23 - BackUp access bb permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bb(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BB_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 22> {
        BACKUP_BUS_PMS_CONSTRAIN_BB_W::new(self)
    }
    #[doc = "Bits 24:25 - BackUp access pwm0 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_pwm0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 24> {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_W::new(self)
    }
    #[doc = "Bits 26:27 - BackUp access timergroup permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_timergroup(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 26> {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W::new(self)
    }
    #[doc = "Bits 28:29 - BackUp access timergroup1 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_timergroup1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 28> {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W::new(self)
    }
    #[doc = "Bits 30:31 - BackUp access systimer permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_systimer(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, 30> {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_constrain_2::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_constrain_2::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_2 to value 0xffcf_fff3"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffcf_fff3;
}
