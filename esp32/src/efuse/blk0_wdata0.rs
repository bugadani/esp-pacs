#[doc = "Register `BLK0_WDATA0` reader"]
pub type R = crate::R<BLK0_WDATA0_SPEC>;
#[doc = "Register `BLK0_WDATA0` writer"]
pub type W = crate::W<BLK0_WDATA0_SPEC>;
#[doc = "Field `WR_DIS` reader - "]
pub type WR_DIS_R = crate::FieldReader<u16>;
#[doc = "Field `WR_DIS` writer - "]
pub type WR_DIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `RD_DIS` reader - "]
pub type RD_DIS_R = crate::FieldReader;
#[doc = "Field `RD_DIS` writer - "]
pub type RD_DIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FLASH_CRYPT_CNT` reader - "]
pub type FLASH_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `FLASH_CRYPT_CNT` writer - "]
pub type FLASH_CRYPT_CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:26"]
    #[inline(always)]
    pub fn flash_crypt_cnt(&self) -> FLASH_CRYPT_CNT_R {
        FLASH_CRYPT_CNT_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA0")
            .field("wr_dis", &format_args!("{}", self.wr_dis().bits()))
            .field("rd_dis", &format_args!("{}", self.rd_dis().bits()))
            .field(
                "flash_crypt_cnt",
                &format_args!("{}", self.flash_crypt_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_WDATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wr_dis(&mut self) -> WR_DIS_W<BLK0_WDATA0_SPEC, 0> {
        WR_DIS_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn rd_dis(&mut self) -> RD_DIS_W<BLK0_WDATA0_SPEC, 16> {
        RD_DIS_W::new(self)
    }
    #[doc = "Bits 20:26"]
    #[inline(always)]
    #[must_use]
    pub fn flash_crypt_cnt(&mut self) -> FLASH_CRYPT_CNT_W<BLK0_WDATA0_SPEC, 20> {
        FLASH_CRYPT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_WDATA0_SPEC;
impl crate::RegisterSpec for BLK0_WDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_wdata0::R`](R) reader structure"]
impl crate::Readable for BLK0_WDATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_wdata0::W`](W) writer structure"]
impl crate::Writable for BLK0_WDATA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA0 to value 0"]
impl crate::Resettable for BLK0_WDATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
