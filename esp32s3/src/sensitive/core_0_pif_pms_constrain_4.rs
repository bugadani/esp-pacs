#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_4` reader"]
pub type R = crate::R<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_4` writer"]
pub type W = crate::W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE` reader - Core0 access usb_device permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE` writer - Core0 access usb_device permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP` reader - Core0 access usb_wrap permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP` writer - Core0 access usb_wrap permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI` reader - Core0 access crypto_peri permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI` writer - Core0 access crypto_peri permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA` reader - Core0 access crypto_dma permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA` writer - Core0 access crypto_dma permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC` reader - Core0 access apb_adc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC` writer - Core0 access apb_adc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM` reader - Core0 access lcd_cam permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM` writer - Core0 access lcd_cam permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR` reader - Core0 access bt_pwr permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR` writer - Core0 access bt_pwr permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB` reader - Core0 access usb permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB` writer - Core0 access usb permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM` reader - Core0 access system permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM` writer - Core0 access system permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE` reader - Core0 access sensitive permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE` writer - Core0 access sensitive permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT` reader - Core0 access interrupt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT` writer - Core0 access interrupt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY` reader - Core0 access dma_copy permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY` writer - Core0 access dma_copy permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG` reader - Core0 access cache_config permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG` writer - Core0 access cache_config permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD` reader - Core0 access ad permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD` writer - Core0 access ad permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO` reader - Core0 access dio permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO` writer - Core0 access dio permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER` reader - Core0 access world_controller permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER` writer - Core0 access world_controller permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core0 access usb_device permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_usb_device(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Core0 access usb_wrap permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_usb_wrap(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core0 access crypto_peri permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_crypto_peri(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core0 access crypto_dma permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_crypto_dma(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core0 access apb_adc permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_apb_adc(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core0 access lcd_cam permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_lcd_cam(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Core0 access bt_pwr permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bt_pwr(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core0 access usb permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_usb(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core0 access system permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_system(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Core0 access sensitive permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_sensitive(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Core0 access interrupt permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_interrupt(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Core0 access dma_copy permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_dma_copy(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Core0 access cache_config permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_cache_config(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core0 access ad permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_ad(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core0 access dio permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_dio(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Core0 access world_controller permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_world_controller(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_4")
            .field(
                "core_0_pif_pms_constrain_world_0_usb_device",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_usb_device().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_usb_wrap",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_usb_wrap().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_crypto_peri",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_crypto_peri().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_crypto_dma",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_crypto_dma().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_apb_adc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_apb_adc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_lcd_cam",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_lcd_cam().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_bt_pwr",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_bt_pwr().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_usb",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_usb().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_system",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_system().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_sensitive",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_sensitive().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_interrupt",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_interrupt().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_dma_copy",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_dma_copy().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_cache_config",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_cache_config().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_ad",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_ad().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_dio",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_dio().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_world_controller",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_world_controller()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core0 access usb_device permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_usb_device(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W::new(self)
    }
    #[doc = "Bits 2:3 - Core0 access usb_wrap permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_usb_wrap(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 2> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W::new(self)
    }
    #[doc = "Bits 4:5 - Core0 access crypto_peri permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_crypto_peri(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W::new(self)
    }
    #[doc = "Bits 6:7 - Core0 access crypto_dma permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_crypto_dma(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W::new(self)
    }
    #[doc = "Bits 8:9 - Core0 access apb_adc permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_apb_adc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W::new(self)
    }
    #[doc = "Bits 10:11 - Core0 access lcd_cam permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_lcd_cam(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_W::new(self)
    }
    #[doc = "Bits 12:13 - Core0 access bt_pwr permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_bt_pwr(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 12> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W::new(self)
    }
    #[doc = "Bits 14:15 - Core0 access usb permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_usb(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_W::new(self)
    }
    #[doc = "Bits 16:17 - Core0 access system permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_system(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W::new(self)
    }
    #[doc = "Bits 18:19 - Core0 access sensitive permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_sensitive(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 18> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W::new(self)
    }
    #[doc = "Bits 20:21 - Core0 access interrupt permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_interrupt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 20> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W::new(self)
    }
    #[doc = "Bits 22:23 - Core0 access dma_copy permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_dma_copy(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 22> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W::new(self)
    }
    #[doc = "Bits 24:25 - Core0 access cache_config permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_cache_config(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 24> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W::new(self)
    }
    #[doc = "Bits 26:27 - Core0 access ad permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_ad(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_W::new(self)
    }
    #[doc = "Bits 28:29 - Core0 access dio permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_dio(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W::new(self)
    }
    #[doc = "Bits 30:31 - Core0 access world_controller permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_world_controller(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC, 30>
    {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core0 access peripherals permission configuration register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_PIF_PMS_CONSTRAIN_4_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_pif_pms_constrain_4::R`](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_pif_pms_constrain_4::W`](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_4 to value 0xffff_ffff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
