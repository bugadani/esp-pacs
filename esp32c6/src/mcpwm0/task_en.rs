#[doc = "Register `TASK_EN` reader"]
pub type R = crate::R<TASK_EN_SPEC>;
#[doc = "Register `TASK_EN` writer"]
pub type W = crate::W<TASK_EN_SPEC>;
#[doc = "Field `TASK_CMPR0_A_UP_EN` reader - set this bit high to enable PWM generator0 timer stamp A's shadow register update task receive"]
pub type TASK_CMPR0_A_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_CMPR0_A_UP_EN` writer - set this bit high to enable PWM generator0 timer stamp A's shadow register update task receive"]
pub type TASK_CMPR0_A_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CMPR1_A_UP_EN` reader - set this bit high to enable PWM generator1 timer stamp A's shadow register update task receive"]
pub type TASK_CMPR1_A_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_CMPR1_A_UP_EN` writer - set this bit high to enable PWM generator1 timer stamp A's shadow register update task receive"]
pub type TASK_CMPR1_A_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CMPR2_A_UP_EN` reader - set this bit high to enable PWM generator2 timer stamp A's shadow register update task receive"]
pub type TASK_CMPR2_A_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_CMPR2_A_UP_EN` writer - set this bit high to enable PWM generator2 timer stamp A's shadow register update task receive"]
pub type TASK_CMPR2_A_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CMPR0_B_UP_EN` reader - set this bit high to enable PWM generator0 timer stamp B's shadow register update task receive"]
pub type TASK_CMPR0_B_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_CMPR0_B_UP_EN` writer - set this bit high to enable PWM generator0 timer stamp B's shadow register update task receive"]
pub type TASK_CMPR0_B_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CMPR1_B_UP_EN` reader - set this bit high to enable PWM generator1 timer stamp B's shadow register update task receive"]
pub type TASK_CMPR1_B_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_CMPR1_B_UP_EN` writer - set this bit high to enable PWM generator1 timer stamp B's shadow register update task receive"]
pub type TASK_CMPR1_B_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CMPR2_B_UP_EN` reader - set this bit high to enable PWM generator2 timer stamp B's shadow register update task receive"]
pub type TASK_CMPR2_B_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_CMPR2_B_UP_EN` writer - set this bit high to enable PWM generator2 timer stamp B's shadow register update task receive"]
pub type TASK_CMPR2_B_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_GEN_STOP_EN` reader - set this bit high to enable all PWM generate stop task receive"]
pub type TASK_GEN_STOP_EN_R = crate::BitReader;
#[doc = "Field `TASK_GEN_STOP_EN` writer - set this bit high to enable all PWM generate stop task receive"]
pub type TASK_GEN_STOP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TIMER0_SYNC_EN` reader - set this bit high to enable timer0 sync task receive"]
pub type TASK_TIMER0_SYNC_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER0_SYNC_EN` writer - set this bit high to enable timer0 sync task receive"]
pub type TASK_TIMER0_SYNC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TIMER1_SYNC_EN` reader - set this bit high to enable timer1 sync task receive"]
pub type TASK_TIMER1_SYNC_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER1_SYNC_EN` writer - set this bit high to enable timer1 sync task receive"]
pub type TASK_TIMER1_SYNC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TIMER2_SYNC_EN` reader - set this bit high to enable timer2 sync task receive"]
pub type TASK_TIMER2_SYNC_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER2_SYNC_EN` writer - set this bit high to enable timer2 sync task receive"]
pub type TASK_TIMER2_SYNC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TIMER0_PERIOD_UP_EN` reader - set this bit high to enable timer0 period update task receive"]
pub type TASK_TIMER0_PERIOD_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER0_PERIOD_UP_EN` writer - set this bit high to enable timer0 period update task receive"]
pub type TASK_TIMER0_PERIOD_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TIMER1_PERIOD_UP_EN` reader - set this bit high to enable timer1 period update task receive"]
pub type TASK_TIMER1_PERIOD_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER1_PERIOD_UP_EN` writer - set this bit high to enable timer1 period update task receive"]
pub type TASK_TIMER1_PERIOD_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TIMER2_PERIOD_UP_EN` reader - set this bit high to enable timer2 period update task receive"]
pub type TASK_TIMER2_PERIOD_UP_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER2_PERIOD_UP_EN` writer - set this bit high to enable timer2 period update task receive"]
pub type TASK_TIMER2_PERIOD_UP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TZ0_OST_EN` reader - set this bit high to enable one shot trip0 task receive"]
pub type TASK_TZ0_OST_EN_R = crate::BitReader;
#[doc = "Field `TASK_TZ0_OST_EN` writer - set this bit high to enable one shot trip0 task receive"]
pub type TASK_TZ0_OST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TZ1_OST_EN` reader - set this bit high to enable one shot trip1 task receive"]
pub type TASK_TZ1_OST_EN_R = crate::BitReader;
#[doc = "Field `TASK_TZ1_OST_EN` writer - set this bit high to enable one shot trip1 task receive"]
pub type TASK_TZ1_OST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_TZ2_OST_EN` reader - set this bit high to enable one shot trip2 task receive"]
pub type TASK_TZ2_OST_EN_R = crate::BitReader;
#[doc = "Field `TASK_TZ2_OST_EN` writer - set this bit high to enable one shot trip2 task receive"]
pub type TASK_TZ2_OST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CLR0_OST_EN` reader - set this bit high to enable one shot trip0 clear task receive"]
pub type TASK_CLR0_OST_EN_R = crate::BitReader;
#[doc = "Field `TASK_CLR0_OST_EN` writer - set this bit high to enable one shot trip0 clear task receive"]
pub type TASK_CLR0_OST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CLR1_OST_EN` reader - set this bit high to enable one shot trip1 clear task receive"]
pub type TASK_CLR1_OST_EN_R = crate::BitReader;
#[doc = "Field `TASK_CLR1_OST_EN` writer - set this bit high to enable one shot trip1 clear task receive"]
pub type TASK_CLR1_OST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CLR2_OST_EN` reader - set this bit high to enable one shot trip2 clear task receive"]
pub type TASK_CLR2_OST_EN_R = crate::BitReader;
#[doc = "Field `TASK_CLR2_OST_EN` writer - set this bit high to enable one shot trip2 clear task receive"]
pub type TASK_CLR2_OST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CAP0_EN` reader - set this bit high to enable capture0 task receive"]
pub type TASK_CAP0_EN_R = crate::BitReader;
#[doc = "Field `TASK_CAP0_EN` writer - set this bit high to enable capture0 task receive"]
pub type TASK_CAP0_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CAP1_EN` reader - set this bit high to enable capture1 task receive"]
pub type TASK_CAP1_EN_R = crate::BitReader;
#[doc = "Field `TASK_CAP1_EN` writer - set this bit high to enable capture1 task receive"]
pub type TASK_CAP1_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASK_CAP2_EN` reader - set this bit high to enable capture2 task receive"]
pub type TASK_CAP2_EN_R = crate::BitReader;
#[doc = "Field `TASK_CAP2_EN` writer - set this bit high to enable capture2 task receive"]
pub type TASK_CAP2_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - set this bit high to enable PWM generator0 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn task_cmpr0_a_up_en(&self) -> TASK_CMPR0_A_UP_EN_R {
        TASK_CMPR0_A_UP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit high to enable PWM generator1 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn task_cmpr1_a_up_en(&self) -> TASK_CMPR1_A_UP_EN_R {
        TASK_CMPR1_A_UP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - set this bit high to enable PWM generator2 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn task_cmpr2_a_up_en(&self) -> TASK_CMPR2_A_UP_EN_R {
        TASK_CMPR2_A_UP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - set this bit high to enable PWM generator0 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn task_cmpr0_b_up_en(&self) -> TASK_CMPR0_B_UP_EN_R {
        TASK_CMPR0_B_UP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit high to enable PWM generator1 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn task_cmpr1_b_up_en(&self) -> TASK_CMPR1_B_UP_EN_R {
        TASK_CMPR1_B_UP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - set this bit high to enable PWM generator2 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn task_cmpr2_b_up_en(&self) -> TASK_CMPR2_B_UP_EN_R {
        TASK_CMPR2_B_UP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - set this bit high to enable all PWM generate stop task receive"]
    #[inline(always)]
    pub fn task_gen_stop_en(&self) -> TASK_GEN_STOP_EN_R {
        TASK_GEN_STOP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set this bit high to enable timer0 sync task receive"]
    #[inline(always)]
    pub fn task_timer0_sync_en(&self) -> TASK_TIMER0_SYNC_EN_R {
        TASK_TIMER0_SYNC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - set this bit high to enable timer1 sync task receive"]
    #[inline(always)]
    pub fn task_timer1_sync_en(&self) -> TASK_TIMER1_SYNC_EN_R {
        TASK_TIMER1_SYNC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - set this bit high to enable timer2 sync task receive"]
    #[inline(always)]
    pub fn task_timer2_sync_en(&self) -> TASK_TIMER2_SYNC_EN_R {
        TASK_TIMER2_SYNC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - set this bit high to enable timer0 period update task receive"]
    #[inline(always)]
    pub fn task_timer0_period_up_en(&self) -> TASK_TIMER0_PERIOD_UP_EN_R {
        TASK_TIMER0_PERIOD_UP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - set this bit high to enable timer1 period update task receive"]
    #[inline(always)]
    pub fn task_timer1_period_up_en(&self) -> TASK_TIMER1_PERIOD_UP_EN_R {
        TASK_TIMER1_PERIOD_UP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - set this bit high to enable timer2 period update task receive"]
    #[inline(always)]
    pub fn task_timer2_period_up_en(&self) -> TASK_TIMER2_PERIOD_UP_EN_R {
        TASK_TIMER2_PERIOD_UP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - set this bit high to enable one shot trip0 task receive"]
    #[inline(always)]
    pub fn task_tz0_ost_en(&self) -> TASK_TZ0_OST_EN_R {
        TASK_TZ0_OST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - set this bit high to enable one shot trip1 task receive"]
    #[inline(always)]
    pub fn task_tz1_ost_en(&self) -> TASK_TZ1_OST_EN_R {
        TASK_TZ1_OST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - set this bit high to enable one shot trip2 task receive"]
    #[inline(always)]
    pub fn task_tz2_ost_en(&self) -> TASK_TZ2_OST_EN_R {
        TASK_TZ2_OST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - set this bit high to enable one shot trip0 clear task receive"]
    #[inline(always)]
    pub fn task_clr0_ost_en(&self) -> TASK_CLR0_OST_EN_R {
        TASK_CLR0_OST_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - set this bit high to enable one shot trip1 clear task receive"]
    #[inline(always)]
    pub fn task_clr1_ost_en(&self) -> TASK_CLR1_OST_EN_R {
        TASK_CLR1_OST_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - set this bit high to enable one shot trip2 clear task receive"]
    #[inline(always)]
    pub fn task_clr2_ost_en(&self) -> TASK_CLR2_OST_EN_R {
        TASK_CLR2_OST_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - set this bit high to enable capture0 task receive"]
    #[inline(always)]
    pub fn task_cap0_en(&self) -> TASK_CAP0_EN_R {
        TASK_CAP0_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - set this bit high to enable capture1 task receive"]
    #[inline(always)]
    pub fn task_cap1_en(&self) -> TASK_CAP1_EN_R {
        TASK_CAP1_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - set this bit high to enable capture2 task receive"]
    #[inline(always)]
    pub fn task_cap2_en(&self) -> TASK_CAP2_EN_R {
        TASK_CAP2_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_EN")
            .field(
                "task_cmpr0_a_up_en",
                &format_args!("{}", self.task_cmpr0_a_up_en().bit()),
            )
            .field(
                "task_cmpr1_a_up_en",
                &format_args!("{}", self.task_cmpr1_a_up_en().bit()),
            )
            .field(
                "task_cmpr2_a_up_en",
                &format_args!("{}", self.task_cmpr2_a_up_en().bit()),
            )
            .field(
                "task_cmpr0_b_up_en",
                &format_args!("{}", self.task_cmpr0_b_up_en().bit()),
            )
            .field(
                "task_cmpr1_b_up_en",
                &format_args!("{}", self.task_cmpr1_b_up_en().bit()),
            )
            .field(
                "task_cmpr2_b_up_en",
                &format_args!("{}", self.task_cmpr2_b_up_en().bit()),
            )
            .field(
                "task_gen_stop_en",
                &format_args!("{}", self.task_gen_stop_en().bit()),
            )
            .field(
                "task_timer0_sync_en",
                &format_args!("{}", self.task_timer0_sync_en().bit()),
            )
            .field(
                "task_timer1_sync_en",
                &format_args!("{}", self.task_timer1_sync_en().bit()),
            )
            .field(
                "task_timer2_sync_en",
                &format_args!("{}", self.task_timer2_sync_en().bit()),
            )
            .field(
                "task_timer0_period_up_en",
                &format_args!("{}", self.task_timer0_period_up_en().bit()),
            )
            .field(
                "task_timer1_period_up_en",
                &format_args!("{}", self.task_timer1_period_up_en().bit()),
            )
            .field(
                "task_timer2_period_up_en",
                &format_args!("{}", self.task_timer2_period_up_en().bit()),
            )
            .field(
                "task_tz0_ost_en",
                &format_args!("{}", self.task_tz0_ost_en().bit()),
            )
            .field(
                "task_tz1_ost_en",
                &format_args!("{}", self.task_tz1_ost_en().bit()),
            )
            .field(
                "task_tz2_ost_en",
                &format_args!("{}", self.task_tz2_ost_en().bit()),
            )
            .field(
                "task_clr0_ost_en",
                &format_args!("{}", self.task_clr0_ost_en().bit()),
            )
            .field(
                "task_clr1_ost_en",
                &format_args!("{}", self.task_clr1_ost_en().bit()),
            )
            .field(
                "task_clr2_ost_en",
                &format_args!("{}", self.task_clr2_ost_en().bit()),
            )
            .field(
                "task_cap0_en",
                &format_args!("{}", self.task_cap0_en().bit()),
            )
            .field(
                "task_cap1_en",
                &format_args!("{}", self.task_cap1_en().bit()),
            )
            .field(
                "task_cap2_en",
                &format_args!("{}", self.task_cap2_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TASK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - set this bit high to enable PWM generator0 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cmpr0_a_up_en(&mut self) -> TASK_CMPR0_A_UP_EN_W<TASK_EN_SPEC, 0> {
        TASK_CMPR0_A_UP_EN_W::new(self)
    }
    #[doc = "Bit 1 - set this bit high to enable PWM generator1 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cmpr1_a_up_en(&mut self) -> TASK_CMPR1_A_UP_EN_W<TASK_EN_SPEC, 1> {
        TASK_CMPR1_A_UP_EN_W::new(self)
    }
    #[doc = "Bit 2 - set this bit high to enable PWM generator2 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cmpr2_a_up_en(&mut self) -> TASK_CMPR2_A_UP_EN_W<TASK_EN_SPEC, 2> {
        TASK_CMPR2_A_UP_EN_W::new(self)
    }
    #[doc = "Bit 3 - set this bit high to enable PWM generator0 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cmpr0_b_up_en(&mut self) -> TASK_CMPR0_B_UP_EN_W<TASK_EN_SPEC, 3> {
        TASK_CMPR0_B_UP_EN_W::new(self)
    }
    #[doc = "Bit 4 - set this bit high to enable PWM generator1 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cmpr1_b_up_en(&mut self) -> TASK_CMPR1_B_UP_EN_W<TASK_EN_SPEC, 4> {
        TASK_CMPR1_B_UP_EN_W::new(self)
    }
    #[doc = "Bit 5 - set this bit high to enable PWM generator2 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cmpr2_b_up_en(&mut self) -> TASK_CMPR2_B_UP_EN_W<TASK_EN_SPEC, 5> {
        TASK_CMPR2_B_UP_EN_W::new(self)
    }
    #[doc = "Bit 6 - set this bit high to enable all PWM generate stop task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_gen_stop_en(&mut self) -> TASK_GEN_STOP_EN_W<TASK_EN_SPEC, 6> {
        TASK_GEN_STOP_EN_W::new(self)
    }
    #[doc = "Bit 7 - set this bit high to enable timer0 sync task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_sync_en(&mut self) -> TASK_TIMER0_SYNC_EN_W<TASK_EN_SPEC, 7> {
        TASK_TIMER0_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 8 - set this bit high to enable timer1 sync task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_sync_en(&mut self) -> TASK_TIMER1_SYNC_EN_W<TASK_EN_SPEC, 8> {
        TASK_TIMER1_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 9 - set this bit high to enable timer2 sync task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_sync_en(&mut self) -> TASK_TIMER2_SYNC_EN_W<TASK_EN_SPEC, 9> {
        TASK_TIMER2_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 10 - set this bit high to enable timer0 period update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_period_up_en(&mut self) -> TASK_TIMER0_PERIOD_UP_EN_W<TASK_EN_SPEC, 10> {
        TASK_TIMER0_PERIOD_UP_EN_W::new(self)
    }
    #[doc = "Bit 11 - set this bit high to enable timer1 period update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_period_up_en(&mut self) -> TASK_TIMER1_PERIOD_UP_EN_W<TASK_EN_SPEC, 11> {
        TASK_TIMER1_PERIOD_UP_EN_W::new(self)
    }
    #[doc = "Bit 12 - set this bit high to enable timer2 period update task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_period_up_en(&mut self) -> TASK_TIMER2_PERIOD_UP_EN_W<TASK_EN_SPEC, 12> {
        TASK_TIMER2_PERIOD_UP_EN_W::new(self)
    }
    #[doc = "Bit 13 - set this bit high to enable one shot trip0 task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_tz0_ost_en(&mut self) -> TASK_TZ0_OST_EN_W<TASK_EN_SPEC, 13> {
        TASK_TZ0_OST_EN_W::new(self)
    }
    #[doc = "Bit 14 - set this bit high to enable one shot trip1 task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_tz1_ost_en(&mut self) -> TASK_TZ1_OST_EN_W<TASK_EN_SPEC, 14> {
        TASK_TZ1_OST_EN_W::new(self)
    }
    #[doc = "Bit 15 - set this bit high to enable one shot trip2 task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_tz2_ost_en(&mut self) -> TASK_TZ2_OST_EN_W<TASK_EN_SPEC, 15> {
        TASK_TZ2_OST_EN_W::new(self)
    }
    #[doc = "Bit 16 - set this bit high to enable one shot trip0 clear task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_clr0_ost_en(&mut self) -> TASK_CLR0_OST_EN_W<TASK_EN_SPEC, 16> {
        TASK_CLR0_OST_EN_W::new(self)
    }
    #[doc = "Bit 17 - set this bit high to enable one shot trip1 clear task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_clr1_ost_en(&mut self) -> TASK_CLR1_OST_EN_W<TASK_EN_SPEC, 17> {
        TASK_CLR1_OST_EN_W::new(self)
    }
    #[doc = "Bit 18 - set this bit high to enable one shot trip2 clear task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_clr2_ost_en(&mut self) -> TASK_CLR2_OST_EN_W<TASK_EN_SPEC, 18> {
        TASK_CLR2_OST_EN_W::new(self)
    }
    #[doc = "Bit 19 - set this bit high to enable capture0 task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cap0_en(&mut self) -> TASK_CAP0_EN_W<TASK_EN_SPEC, 19> {
        TASK_CAP0_EN_W::new(self)
    }
    #[doc = "Bit 20 - set this bit high to enable capture1 task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cap1_en(&mut self) -> TASK_CAP1_EN_W<TASK_EN_SPEC, 20> {
        TASK_CAP1_EN_W::new(self)
    }
    #[doc = "Bit 21 - set this bit high to enable capture2 task receive"]
    #[inline(always)]
    #[must_use]
    pub fn task_cap2_en(&mut self) -> TASK_CAP2_EN_W<TASK_EN_SPEC, 21> {
        TASK_CAP2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MCPWM task enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_EN_SPEC;
impl crate::RegisterSpec for TASK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_en::R`](R) reader structure"]
impl crate::Readable for TASK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_en::W`](W) writer structure"]
impl crate::Writable for TASK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASK_EN to value 0"]
impl crate::Resettable for TASK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
