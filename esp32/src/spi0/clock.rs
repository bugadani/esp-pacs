#[doc = "Register `CLOCK` reader"]
pub type R = crate::R<CLOCK_SPEC>;
#[doc = "Register `CLOCK` writer"]
pub type W = crate::W<CLOCK_SPEC>;
#[doc = "Field `CLKCNT_L` reader - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0."]
pub type CLKCNT_L_R = crate::FieldReader;
#[doc = "Field `CLKCNT_L` writer - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0."]
pub type CLKCNT_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CLKCNT_H` reader - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0."]
pub type CLKCNT_H_R = crate::FieldReader;
#[doc = "Field `CLKCNT_H` writer - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0."]
pub type CLKCNT_H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CLKCNT_N` reader - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
pub type CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `CLKCNT_N` writer - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
pub type CLKCNT_N_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CLKDIV_PRE` reader - In the master mode it is pre-divider of spi_clk."]
pub type CLKDIV_PRE_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV_PRE` writer - In the master mode it is pre-divider of spi_clk."]
pub type CLKDIV_PRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `CLK_EQU_SYSCLK` reader - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock."]
pub type CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `CLK_EQU_SYSCLK` writer - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock."]
pub type CLK_EQU_SYSCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0."]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0."]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:30 - In the master mode it is pre-divider of spi_clk."]
    #[inline(always)]
    pub fn clkdiv_pre(&self) -> CLKDIV_PRE_R {
        CLKDIV_PRE_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK")
            .field("clkcnt_l", &format_args!("{}", self.clkcnt_l().bits()))
            .field("clkcnt_h", &format_args!("{}", self.clkcnt_h().bits()))
            .field("clkcnt_n", &format_args!("{}", self.clkcnt_n().bits()))
            .field("clkdiv_pre", &format_args!("{}", self.clkdiv_pre().bits()))
            .field(
                "clk_equ_sysclk",
                &format_args!("{}", self.clk_equ_sysclk().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W<CLOCK_SPEC, 0> {
        CLKCNT_L_W::new(self)
    }
    #[doc = "Bits 6:11 - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W<CLOCK_SPEC, 6> {
        CLKCNT_H_W::new(self)
    }
    #[doc = "Bits 12:17 - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W<CLOCK_SPEC, 12> {
        CLKCNT_N_W::new(self)
    }
    #[doc = "Bits 18:30 - In the master mode it is pre-divider of spi_clk."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv_pre(&mut self) -> CLKDIV_PRE_W<CLOCK_SPEC, 18> {
        CLKDIV_PRE_W::new(self)
    }
    #[doc = "Bit 31 - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock."]
    #[inline(always)]
    #[must_use]
    pub fn clk_equ_sysclk(&mut self) -> CLK_EQU_SYSCLK_W<CLOCK_SPEC, 31> {
        CLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock::R`](R) reader structure"]
impl crate::Readable for CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock::W`](W) writer structure"]
impl crate::Writable for CLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK to value 0x8000_3043"]
impl crate::Resettable for CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_3043;
}
