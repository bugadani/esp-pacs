#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `CHECK_SUM_EN` reader - Set this bit to enable head checksum check when receiving."]
pub type CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `CHECK_SUM_EN` writer - Set this bit to enable head checksum check when receiving."]
pub type CHECK_SUM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHECK_SEQ_EN` reader - Set this bit to enable sequence number check when receiving."]
pub type CHECK_SEQ_EN_R = crate::BitReader;
#[doc = "Field `CHECK_SEQ_EN` writer - Set this bit to enable sequence number check when receiving."]
pub type CHECK_SEQ_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC_DISABLE` reader - Set this bit to support CRC calculation, and data integrity check bit should 1."]
pub type CRC_DISABLE_R = crate::BitReader;
#[doc = "Field `CRC_DISABLE` writer - Set this bit to support CRC calculation, and data integrity check bit should 1."]
pub type CRC_DISABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAVE_HEAD` reader - Set this bit to save data packet head when UHCI receive data."]
pub type SAVE_HEAD_R = crate::BitReader;
#[doc = "Field `SAVE_HEAD` writer - Set this bit to save data packet head when UHCI receive data."]
pub type SAVE_HEAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_CHECK_SUM_RE` reader - Set this bit to encode data packet with checksum."]
pub type TX_CHECK_SUM_RE_R = crate::BitReader;
#[doc = "Field `TX_CHECK_SUM_RE` writer - Set this bit to encode data packet with checksum."]
pub type TX_CHECK_SUM_RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_ACK_NUM_RE` reader - Set this bit to encode data packet with ACK when reliable data packet is ready."]
pub type TX_ACK_NUM_RE_R = crate::BitReader;
#[doc = "Field `TX_ACK_NUM_RE` writer - Set this bit to encode data packet with ACK when reliable data packet is ready."]
pub type TX_ACK_NUM_RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAIT_SW_START` reader - Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status."]
pub type WAIT_SW_START_R = crate::BitReader;
#[doc = "Field `WAIT_SW_START` writer - Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status."]
pub type WAIT_SW_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SW_START` writer - Set this bit to transmit data packet if UCHI_ENCODE_STATE is ST_SW_WAIT."]
pub type SW_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable head checksum check when receiving."]
    #[inline(always)]
    pub fn check_sum_en(&self) -> CHECK_SUM_EN_R {
        CHECK_SUM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable sequence number check when receiving."]
    #[inline(always)]
    pub fn check_seq_en(&self) -> CHECK_SEQ_EN_R {
        CHECK_SEQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to support CRC calculation, and data integrity check bit should 1."]
    #[inline(always)]
    pub fn crc_disable(&self) -> CRC_DISABLE_R {
        CRC_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to save data packet head when UHCI receive data."]
    #[inline(always)]
    pub fn save_head(&self) -> SAVE_HEAD_R {
        SAVE_HEAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to encode data packet with checksum."]
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> TX_CHECK_SUM_RE_R {
        TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to encode data packet with ACK when reliable data packet is ready."]
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> TX_ACK_NUM_RE_R {
        TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status."]
    #[inline(always)]
    pub fn wait_sw_start(&self) -> WAIT_SW_START_R {
        WAIT_SW_START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field(
                "check_sum_en",
                &format_args!("{}", self.check_sum_en().bit()),
            )
            .field(
                "check_seq_en",
                &format_args!("{}", self.check_seq_en().bit()),
            )
            .field("crc_disable", &format_args!("{}", self.crc_disable().bit()))
            .field("save_head", &format_args!("{}", self.save_head().bit()))
            .field(
                "tx_check_sum_re",
                &format_args!("{}", self.tx_check_sum_re().bit()),
            )
            .field(
                "tx_ack_num_re",
                &format_args!("{}", self.tx_ack_num_re().bit()),
            )
            .field(
                "wait_sw_start",
                &format_args!("{}", self.wait_sw_start().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable head checksum check when receiving."]
    #[inline(always)]
    #[must_use]
    pub fn check_sum_en(&mut self) -> CHECK_SUM_EN_W<CONF1_SPEC, 0> {
        CHECK_SUM_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to enable sequence number check when receiving."]
    #[inline(always)]
    #[must_use]
    pub fn check_seq_en(&mut self) -> CHECK_SEQ_EN_W<CONF1_SPEC, 1> {
        CHECK_SEQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to support CRC calculation, and data integrity check bit should 1."]
    #[inline(always)]
    #[must_use]
    pub fn crc_disable(&mut self) -> CRC_DISABLE_W<CONF1_SPEC, 2> {
        CRC_DISABLE_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to save data packet head when UHCI receive data."]
    #[inline(always)]
    #[must_use]
    pub fn save_head(&mut self) -> SAVE_HEAD_W<CONF1_SPEC, 3> {
        SAVE_HEAD_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to encode data packet with checksum."]
    #[inline(always)]
    #[must_use]
    pub fn tx_check_sum_re(&mut self) -> TX_CHECK_SUM_RE_W<CONF1_SPEC, 4> {
        TX_CHECK_SUM_RE_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to encode data packet with ACK when reliable data packet is ready."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack_num_re(&mut self) -> TX_ACK_NUM_RE_W<CONF1_SPEC, 5> {
        TX_ACK_NUM_RE_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to enable UHCI encoder transfer to ST_SW_WAIT status."]
    #[inline(always)]
    #[must_use]
    pub fn wait_sw_start(&mut self) -> WAIT_SW_START_W<CONF1_SPEC, 7> {
        WAIT_SW_START_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to transmit data packet if UCHI_ENCODE_STATE is ST_SW_WAIT."]
    #[inline(always)]
    #[must_use]
    pub fn sw_start(&mut self) -> SW_START_W<CONF1_SPEC, 8> {
        SW_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UHCI Configuration Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x33"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x33;
}
