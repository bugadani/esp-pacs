#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `SDA_FORCE_OUT` reader - 0: direct output, 1: open drain output."]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - 0: direct output, 1: open drain output."]
pub type SDA_FORCE_OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCL_FORCE_OUT` reader - 0: direct output, 1: open drain output."]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - 0: direct output, 1: open drain output."]
pub type SCL_FORCE_OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAMPLE_SCL_LEVEL` reader - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
pub type SAMPLE_SCL_LEVEL_R = crate::BitReader;
#[doc = "Field `SAMPLE_SCL_LEVEL` writer - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
pub type SAMPLE_SCL_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_FULL_ACK_LEVEL` reader - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type RX_FULL_ACK_LEVEL_R = crate::BitReader;
#[doc = "Field `RX_FULL_ACK_LEVEL` writer - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type RX_FULL_ACK_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MS_MODE` reader - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
pub type MS_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_START` writer - Set this bit to start sending the data in txfifo."]
pub type TRANS_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_LSB_FIRST` reader - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
pub type TX_LSB_FIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_LSB_FIRST` reader - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
pub type RX_LSB_FIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_EN` reader - Reserved"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Reserved"]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARBITRATION_EN` reader - This is the enable bit for arbitration_lost."]
pub type ARBITRATION_EN_R = crate::BitReader;
#[doc = "Field `ARBITRATION_EN` writer - This is the enable bit for arbitration_lost."]
pub type ARBITRATION_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FSM_RST` writer - This register is used to reset the scl FMS."]
pub type FSM_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CONF_UPGATE` writer - synchronization bit"]
pub type CONF_UPGATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLV_TX_AUTO_START_EN` reader - This is the enable bit for slave to send data automatically"]
pub type SLV_TX_AUTO_START_EN_R = crate::BitReader;
#[doc = "Field `SLV_TX_AUTO_START_EN` writer - This is the enable bit for slave to send data automatically"]
pub type SLV_TX_AUTO_START_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: direct output, 1: open drain output."]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: direct output, 1: open drain output."]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
    #[inline(always)]
    pub fn sample_scl_level(&self) -> SAMPLE_SCL_LEVEL_R {
        SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    pub fn rx_full_ack_level(&self) -> RX_FULL_ACK_LEVEL_R {
        RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the enable bit for arbitration_lost."]
    #[inline(always)]
    pub fn arbitration_en(&self) -> ARBITRATION_EN_R {
        ARBITRATION_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for slave to send data automatically"]
    #[inline(always)]
    pub fn slv_tx_auto_start_en(&self) -> SLV_TX_AUTO_START_EN_R {
        SLV_TX_AUTO_START_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field(
                "sda_force_out",
                &format_args!("{}", self.sda_force_out().bit()),
            )
            .field(
                "scl_force_out",
                &format_args!("{}", self.scl_force_out().bit()),
            )
            .field(
                "sample_scl_level",
                &format_args!("{}", self.sample_scl_level().bit()),
            )
            .field(
                "rx_full_ack_level",
                &format_args!("{}", self.rx_full_ack_level().bit()),
            )
            .field("ms_mode", &format_args!("{}", self.ms_mode().bit()))
            .field(
                "tx_lsb_first",
                &format_args!("{}", self.tx_lsb_first().bit()),
            )
            .field(
                "rx_lsb_first",
                &format_args!("{}", self.rx_lsb_first().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field(
                "arbitration_en",
                &format_args!("{}", self.arbitration_en().bit()),
            )
            .field(
                "slv_tx_auto_start_en",
                &format_args!("{}", self.slv_tx_auto_start_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 0: direct output, 1: open drain output."]
    #[inline(always)]
    #[must_use]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<CTR_SPEC, 0> {
        SDA_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 1 - 0: direct output, 1: open drain output."]
    #[inline(always)]
    #[must_use]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<CTR_SPEC, 1> {
        SCL_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
    #[inline(always)]
    #[must_use]
    pub fn sample_scl_level(&mut self) -> SAMPLE_SCL_LEVEL_W<CTR_SPEC, 2> {
        SAMPLE_SCL_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full_ack_level(&mut self) -> RX_FULL_ACK_LEVEL_W<CTR_SPEC, 3> {
        RX_FULL_ACK_LEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
    #[inline(always)]
    #[must_use]
    pub fn ms_mode(&mut self) -> MS_MODE_W<CTR_SPEC, 4> {
        MS_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to start sending the data in txfifo."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<CTR_SPEC, 5> {
        TRANS_START_W::new(self)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<CTR_SPEC, 6> {
        TX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<CTR_SPEC, 7> {
        RX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CTR_SPEC, 8> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - This is the enable bit for arbitration_lost."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_en(&mut self) -> ARBITRATION_EN_W<CTR_SPEC, 9> {
        ARBITRATION_EN_W::new(self)
    }
    #[doc = "Bit 10 - This register is used to reset the scl FMS."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_rst(&mut self) -> FSM_RST_W<CTR_SPEC, 10> {
        FSM_RST_W::new(self)
    }
    #[doc = "Bit 11 - synchronization bit"]
    #[inline(always)]
    #[must_use]
    pub fn conf_upgate(&mut self) -> CONF_UPGATE_W<CTR_SPEC, 11> {
        CONF_UPGATE_W::new(self)
    }
    #[doc = "Bit 12 - This is the enable bit for slave to send data automatically"]
    #[inline(always)]
    #[must_use]
    pub fn slv_tx_auto_start_en(&mut self) -> SLV_TX_AUTO_START_EN_W<CTR_SPEC, 12> {
        SLV_TX_AUTO_START_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmission setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR to value 0x020b"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x020b;
}
