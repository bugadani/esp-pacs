#[doc = "Register `BLK6_W3` reader"]
pub type R = crate::R<BLK6_W3_SPEC>;
#[doc = "Field `BLOCK6_W3` reader - Otp block6 word3 data."]
pub type BLOCK6_W3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block6 word3 data."]
    #[inline(always)]
    pub fn block6_w3(&self) -> BLOCK6_W3_R {
        BLOCK6_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK6_W3")
            .field("block6_w3", &format_args!("{}", self.block6_w3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK6_W3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block6 data register3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk6_w3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK6_W3_SPEC;
impl crate::RegisterSpec for BLK6_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk6_w3::R`](R) reader structure"]
impl crate::Readable for BLK6_W3_SPEC {}
#[doc = "`reset()` method sets BLK6_W3 to value 0"]
impl crate::Resettable for BLK6_W3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
