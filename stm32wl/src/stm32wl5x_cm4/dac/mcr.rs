///Register `MCR` reader
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCR` writer
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MODE1` reader - DAC Channel 1 mode
pub type MODE1_R = crate::FieldReader<u8, MODE1_A>;
///DAC Channel 1 mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    ///0: Normal mode - DAC channelx is connected to external pin with Buffer enabled
    NormalPinBuffer = 0,
    ///1: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    NormalPinChipBuffer = 1,
    ///2: Normal mode - DAC channelx is connected to external pin with Buffer disabled
    NormalPinNoBuffer = 2,
    ///3: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    NormalChipNoBuffer = 3,
    ///4: S&amp;H mode - DAC channelx is connected to external pin with Buffer enabled
    ShpinBuffer = 4,
    ///5: S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    ShpinChipBuffer = 5,
    ///6: S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    ShpinNoBuffer = 6,
    ///7: S&amp;H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    ShchipNoBuffer = 7,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
impl MODE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::NormalPinBuffer,
            1 => MODE1_A::NormalPinChipBuffer,
            2 => MODE1_A::NormalPinNoBuffer,
            3 => MODE1_A::NormalChipNoBuffer,
            4 => MODE1_A::ShpinBuffer,
            5 => MODE1_A::ShpinChipBuffer,
            6 => MODE1_A::ShpinNoBuffer,
            7 => MODE1_A::ShchipNoBuffer,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NormalPinBuffer`
    #[inline(always)]
    pub fn is_normal_pin_buffer(&self) -> bool {
        *self == MODE1_A::NormalPinBuffer
    }
    ///Checks if the value of the field is `NormalPinChipBuffer`
    #[inline(always)]
    pub fn is_normal_pin_chip_buffer(&self) -> bool {
        *self == MODE1_A::NormalPinChipBuffer
    }
    ///Checks if the value of the field is `NormalPinNoBuffer`
    #[inline(always)]
    pub fn is_normal_pin_no_buffer(&self) -> bool {
        *self == MODE1_A::NormalPinNoBuffer
    }
    ///Checks if the value of the field is `NormalChipNoBuffer`
    #[inline(always)]
    pub fn is_normal_chip_no_buffer(&self) -> bool {
        *self == MODE1_A::NormalChipNoBuffer
    }
    ///Checks if the value of the field is `ShpinBuffer`
    #[inline(always)]
    pub fn is_shpin_buffer(&self) -> bool {
        *self == MODE1_A::ShpinBuffer
    }
    ///Checks if the value of the field is `ShpinChipBuffer`
    #[inline(always)]
    pub fn is_shpin_chip_buffer(&self) -> bool {
        *self == MODE1_A::ShpinChipBuffer
    }
    ///Checks if the value of the field is `ShpinNoBuffer`
    #[inline(always)]
    pub fn is_shpin_no_buffer(&self) -> bool {
        *self == MODE1_A::ShpinNoBuffer
    }
    ///Checks if the value of the field is `ShchipNoBuffer`
    #[inline(always)]
    pub fn is_shchip_no_buffer(&self) -> bool {
        *self == MODE1_A::ShchipNoBuffer
    }
}
///Field `MODE1` writer - DAC Channel 1 mode
pub type MODE1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, MODE1_A, 3, O>;
impl<'a, const O: u8> MODE1_W<'a, O> {
    ///Normal mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalPinBuffer)
    }
    ///Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_chip_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalPinChipBuffer)
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer disabled
    #[inline(always)]
    pub fn normal_pin_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalPinNoBuffer)
    }
    ///Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn normal_chip_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalChipNoBuffer)
    }
    ///S&amp;H mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn shpin_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShpinBuffer)
    }
    ///S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn shpin_chip_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShpinChipBuffer)
    }
    ///S&amp;H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shpin_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShpinNoBuffer)
    }
    ///S&amp;H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shchip_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShchipNoBuffer)
    }
}
impl R {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<0> {
        MODE1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](index.html) module
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcr::R](R) reader structure
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcr::W](W) writer structure
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
