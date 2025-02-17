#[doc = "Register `AT_CMD_CHAR_SYNC` reader"]
pub type R = crate::R<AT_CMD_CHAR_SYNC_SPEC>;
#[doc = "Register `AT_CMD_CHAR_SYNC` writer"]
pub type W = crate::W<AT_CMD_CHAR_SYNC_SPEC>;
#[doc = "Field `AT_CMD_CHAR` reader - This register is used to configure the content of at_cmd char."]
pub type AT_CMD_CHAR_R = crate::FieldReader;
#[doc = "Field `AT_CMD_CHAR` writer - This register is used to configure the content of at_cmd char."]
pub type AT_CMD_CHAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CHAR_NUM` reader - This register is used to configure the num of continuous at_cmd chars received by receiver."]
pub type CHAR_NUM_R = crate::FieldReader;
#[doc = "Field `CHAR_NUM` writer - This register is used to configure the num of continuous at_cmd chars received by receiver."]
pub type CHAR_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    pub fn at_cmd_char(&self) -> AT_CMD_CHAR_R {
        AT_CMD_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register is used to configure the num of continuous at_cmd chars received by receiver."]
    #[inline(always)]
    pub fn char_num(&self) -> CHAR_NUM_R {
        CHAR_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_CHAR_SYNC")
            .field(
                "at_cmd_char",
                &format_args!("{}", self.at_cmd_char().bits()),
            )
            .field("char_num", &format_args!("{}", self.char_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AT_CMD_CHAR_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    #[must_use]
    pub fn at_cmd_char(&mut self) -> AT_CMD_CHAR_W<AT_CMD_CHAR_SYNC_SPEC, 0> {
        AT_CMD_CHAR_W::new(self)
    }
    #[doc = "Bits 8:15 - This register is used to configure the num of continuous at_cmd chars received by receiver."]
    #[inline(always)]
    #[must_use]
    pub fn char_num(&mut self) -> CHAR_NUM_W<AT_CMD_CHAR_SYNC_SPEC, 8> {
        CHAR_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AT escape sequence detection configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_char_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_char_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AT_CMD_CHAR_SYNC_SPEC;
impl crate::RegisterSpec for AT_CMD_CHAR_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_char_sync::R`](R) reader structure"]
impl crate::Readable for AT_CMD_CHAR_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_char_sync::W`](W) writer structure"]
impl crate::Writable for AT_CMD_CHAR_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AT_CMD_CHAR_SYNC to value 0x032b"]
impl crate::Resettable for AT_CMD_CHAR_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x032b;
}
