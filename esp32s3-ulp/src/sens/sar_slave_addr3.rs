#[doc = "Register `SAR_SLAVE_ADDR3` reader"]
pub type R = crate::R<SAR_SLAVE_ADDR3_SPEC>;
#[doc = "Register `SAR_SLAVE_ADDR3` writer"]
pub type W = crate::W<SAR_SLAVE_ADDR3_SPEC>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR5` reader - configure i2c slave address5"]
pub type SAR_I2C_SLAVE_ADDR5_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR5` writer - configure i2c slave address5"]
pub type SAR_I2C_SLAVE_ADDR5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR4` reader - configure i2c slave address4"]
pub type SAR_I2C_SLAVE_ADDR4_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR4` writer - configure i2c slave address4"]
pub type SAR_I2C_SLAVE_ADDR4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - configure i2c slave address5"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr5(&self) -> SAR_I2C_SLAVE_ADDR5_R {
        SAR_I2C_SLAVE_ADDR5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - configure i2c slave address4"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr4(&self) -> SAR_I2C_SLAVE_ADDR4_R {
        SAR_I2C_SLAVE_ADDR4_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR3")
            .field(
                "sar_i2c_slave_addr5",
                &format_args!("{}", self.sar_i2c_slave_addr5().bits()),
            )
            .field(
                "sar_i2c_slave_addr4",
                &format_args!("{}", self.sar_i2c_slave_addr4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_SLAVE_ADDR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - configure i2c slave address5"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_slave_addr5(&mut self) -> SAR_I2C_SLAVE_ADDR5_W<SAR_SLAVE_ADDR3_SPEC, 0> {
        SAR_I2C_SLAVE_ADDR5_W::new(self)
    }
    #[doc = "Bits 11:21 - configure i2c slave address4"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_slave_addr4(&mut self) -> SAR_I2C_SLAVE_ADDR4_W<SAR_SLAVE_ADDR3_SPEC, 11> {
        SAR_I2C_SLAVE_ADDR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure i2c slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SLAVE_ADDR3_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_slave_addr3::R`](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_slave_addr3::W`](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR3 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
