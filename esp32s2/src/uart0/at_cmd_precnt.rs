#[doc = "Register `AT_CMD_PRECNT` reader"]
pub type R = crate::R<AT_CMD_PRECNT_SPEC>;
#[doc = "Register `AT_CMD_PRECNT` writer"]
pub type W = crate::W<AT_CMD_PRECNT_SPEC>;
#[doc = "Field `PRE_IDLE_NUM` reader - This register is used to configure the idle duration time before the first AT_CMD is received by the receiver. It will not take the next data received as AT_CMD character when the duration is less than this register's value."]
pub type PRE_IDLE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PRE_IDLE_NUM` writer - This register is used to configure the idle duration time before the first AT_CMD is received by the receiver. It will not take the next data received as AT_CMD character when the duration is less than this register's value."]
pub type PRE_IDLE_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the idle duration time before the first AT_CMD is received by the receiver. It will not take the next data received as AT_CMD character when the duration is less than this register's value."]
    #[inline(always)]
    pub fn pre_idle_num(&self) -> PRE_IDLE_NUM_R {
        PRE_IDLE_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_PRECNT")
            .field(
                "pre_idle_num",
                &format_args!("{}", self.pre_idle_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AT_CMD_PRECNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the idle duration time before the first AT_CMD is received by the receiver. It will not take the next data received as AT_CMD character when the duration is less than this register's value."]
    #[inline(always)]
    #[must_use]
    pub fn pre_idle_num(&mut self) -> PRE_IDLE_NUM_W<AT_CMD_PRECNT_SPEC, 0> {
        PRE_IDLE_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pre-sequence timing configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_precnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_precnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AT_CMD_PRECNT_SPEC;
impl crate::RegisterSpec for AT_CMD_PRECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_precnt::R`](R) reader structure"]
impl crate::Readable for AT_CMD_PRECNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_precnt::W`](W) writer structure"]
impl crate::Writable for AT_CMD_PRECNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AT_CMD_PRECNT to value 0x0901"]
impl crate::Resettable for AT_CMD_PRECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0901;
}
