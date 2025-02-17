#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SDA_FORCE_OUT` reader - SDA output mode. 0: open drain. 1: push pull."]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - SDA output mode. 0: open drain. 1: push pull."]
pub type SDA_FORCE_OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCL_FORCE_OUT` reader - SCL output mode. 0: open drain. 1: push pull."]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - SCL output mode. 0: open drain. 1: push pull."]
pub type SCL_FORCE_OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MS_MODE` reader - Set this bit to configure RTC I²C as a master."]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - Set this bit to configure RTC I²C as a master."]
pub type MS_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_START` reader - Set this bit to 1, RTC I2C starts sending data."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `TRANS_START` writer - Set this bit to 1, RTC I2C starts sending data."]
pub type TRANS_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_LSB_FIRST` reader - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
pub type TX_LSB_FIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_LSB_FIRST` reader - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
pub type RX_LSB_FIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_GATE_EN` reader - RTC I²C controller clock gate."]
pub type CLK_GATE_EN_R = crate::BitReader;
#[doc = "Field `CLK_GATE_EN` writer - RTC I²C controller clock gate."]
pub type CLK_GATE_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESET` reader - RTC I²C software reset."]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - RTC I²C software reset."]
pub type RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_EN` reader - rtc i2c reg clk gating"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - rtc i2c reg clk gating"]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SDA output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to configure RTC I²C as a master."]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1, RTC I2C starts sending data."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC I²C controller clock gate."]
    #[inline(always)]
    pub fn clk_gate_en(&self) -> CLK_GATE_EN_R {
        CLK_GATE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC I²C software reset."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field(
                "sda_force_out",
                &format_args!("{}", self.sda_force_out().bit()),
            )
            .field(
                "scl_force_out",
                &format_args!("{}", self.scl_force_out().bit()),
            )
            .field("ms_mode", &format_args!("{}", self.ms_mode().bit()))
            .field("trans_start", &format_args!("{}", self.trans_start().bit()))
            .field(
                "tx_lsb_first",
                &format_args!("{}", self.tx_lsb_first().bit()),
            )
            .field(
                "rx_lsb_first",
                &format_args!("{}", self.rx_lsb_first().bit()),
            )
            .field("clk_gate_en", &format_args!("{}", self.clk_gate_en().bit()))
            .field("reset", &format_args!("{}", self.reset().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SDA output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    #[must_use]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<CTRL_SPEC, 0> {
        SDA_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 1 - SCL output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    #[must_use]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<CTRL_SPEC, 1> {
        SCL_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to configure RTC I²C as a master."]
    #[inline(always)]
    #[must_use]
    pub fn ms_mode(&mut self) -> MS_MODE_W<CTRL_SPEC, 2> {
        MS_MODE_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to 1, RTC I2C starts sending data."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<CTRL_SPEC, 3> {
        TRANS_START_W::new(self)
    }
    #[doc = "Bit 4 - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<CTRL_SPEC, 4> {
        TX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<CTRL_SPEC, 5> {
        RX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 29 - RTC I²C controller clock gate."]
    #[inline(always)]
    #[must_use]
    pub fn clk_gate_en(&mut self) -> CLK_GATE_EN_W<CTRL_SPEC, 29> {
        CLK_GATE_EN_W::new(self)
    }
    #[doc = "Bit 30 - RTC I²C software reset."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CTRL_SPEC, 30> {
        RESET_W::new(self)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CTRL_SPEC, 31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmission setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
