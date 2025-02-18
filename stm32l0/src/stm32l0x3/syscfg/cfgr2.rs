///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FWDIS` reader - Firewall disable bit
pub type FWDIS_R = crate::BitReader<FWDIS_A>;
///Firewall disable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWDIS_A {
    ///0: Firewall access enabled
    Enabled = 0,
    ///1: Firewall access disabled
    Disabled = 1,
}
impl From<FWDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FWDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FWDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FWDIS_A {
        match self.bits {
            false => FWDIS_A::Enabled,
            true => FWDIS_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWDIS_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWDIS_A::Disabled
    }
}
///Field `FWDIS` writer - Firewall disable bit
pub type FWDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, FWDIS_A, O>;
impl<'a, const O: u8> FWDIS_W<'a, O> {
    ///Firewall access enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FWDIS_A::Enabled)
    }
    ///Firewall access disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FWDIS_A::Disabled)
    }
}
///Field `I2C_PB6_FMP` reader - Fm+ drive capability on PB6 enable bit
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP_A>;
///Fm+ drive capability on PB6 enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP_A {
    ///0: PB6 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB6_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB6_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB6_FMP_A {
        match self.bits {
            false => I2C_PB6_FMP_A::Standard,
            true => I2C_PB6_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP_A::Fmp
    }
}
///Field `I2C_PB6_FMP` writer - Fm+ drive capability on PB6 enable bit
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2C_PB6_FMP_A, O>;
impl<'a, const O: u8> I2C_PB6_FMP_W<'a, O> {
    ///PB6 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Fmp)
    }
}
///Field `I2C_PB7_FMP` reader - Fm+ drive capability on PB7 enable bit
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP_A>;
///Fm+ drive capability on PB7 enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP_A {
    ///0: PB7 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB7_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB7_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB7_FMP_A {
        match self.bits {
            false => I2C_PB7_FMP_A::Standard,
            true => I2C_PB7_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP_A::Fmp
    }
}
///Field `I2C_PB7_FMP` writer - Fm+ drive capability on PB7 enable bit
pub type I2C_PB7_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2C_PB7_FMP_A, O>;
impl<'a, const O: u8> I2C_PB7_FMP_W<'a, O> {
    ///PB7 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Fmp)
    }
}
///Field `I2C_PB8_FMP` reader - Fm+ drive capability on PB8 enable bit
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP_A>;
///Fm+ drive capability on PB8 enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP_A {
    ///0: PB8 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB8_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB8_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB8_FMP_A {
        match self.bits {
            false => I2C_PB8_FMP_A::Standard,
            true => I2C_PB8_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP_A::Fmp
    }
}
///Field `I2C_PB8_FMP` writer - Fm+ drive capability on PB8 enable bit
pub type I2C_PB8_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2C_PB8_FMP_A, O>;
impl<'a, const O: u8> I2C_PB8_FMP_W<'a, O> {
    ///PB8 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Fmp)
    }
}
///Field `I2C_PB9_FMP` reader - Fm+ drive capability on PB9 enable bit
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP_A>;
///Fm+ drive capability on PB9 enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP_A {
    ///0: PB9 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB9_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB9_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB9_FMP_A {
        match self.bits {
            false => I2C_PB9_FMP_A::Standard,
            true => I2C_PB9_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP_A::Fmp
    }
}
///Field `I2C_PB9_FMP` writer - Fm+ drive capability on PB9 enable bit
pub type I2C_PB9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2C_PB9_FMP_A, O>;
impl<'a, const O: u8> I2C_PB9_FMP_W<'a, O> {
    ///PB9 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Fmp)
    }
}
///Field `I2C1_FMP` reader - I2C1 Fm+ drive capability enable bit
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP_A>;
///I2C1 Fm+ drive capability enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP_A {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    Fmp = 1,
}
impl From<I2C1_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C1_FMP_A {
        match self.bits {
            false => I2C1_FMP_A::Standard,
            true => I2C1_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP_A::Fmp
    }
}
///Field `I2C1_FMP` writer - I2C1 Fm+ drive capability enable bit
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2C1_FMP_A, O>;
impl<'a, const O: u8> I2C1_FMP_W<'a, O> {
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Standard)
    }
    ///FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Fmp)
    }
}
///Field `I2C2_FMP` reader - I2C2 Fm+ drive capability enable bit
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP_A>;
///I2C2 Fm+ drive capability enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_FMP_A {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers
    Fmp = 1,
}
impl From<I2C2_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C2_FMP_A {
        match self.bits {
            false => I2C2_FMP_A::Standard,
            true => I2C2_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C2_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP_A::Fmp
    }
}
///Field `I2C2_FMP` writer - I2C2 Fm+ drive capability enable bit
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2C2_FMP_A, O>;
impl<'a, const O: u8> I2C2_FMP_W<'a, O> {
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Standard)
    }
    ///FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Fmp)
    }
}
///Field `I2C3_FMP` reader - I2C3 Fm+ drive capability enable bit
pub type I2C3_FMP_R = crate::BitReader<I2C3_FMP_A>;
///I2C3 Fm+ drive capability enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3_FMP_A {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers
    Fmp = 1,
}
impl From<I2C3_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C3_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C3_FMP_A {
        match self.bits {
            false => I2C3_FMP_A::Standard,
            true => I2C3_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C3_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C3_FMP_A::Fmp
    }
}
///Field `I2C3_FMP` writer - I2C3 Fm+ drive capability enable bit
pub type I2C3_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, I2C3_FMP_A, O>;
impl<'a, const O: u8> I2C3_FMP_W<'a, O> {
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::Standard)
    }
    ///FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::Fmp)
    }
}
impl R {
    ///Bit 0 - Firewall disable bit
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Fm+ drive capability on PB6 enable bit
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Fm+ drive capability on PB7 enable bit
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Fm+ drive capability on PB8 enable bit
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Fm+ drive capability on PB9 enable bit
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - I2C1 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - I2C2 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - I2C3 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Firewall disable bit
    #[inline(always)]
    #[must_use]
    pub fn fwdis(&mut self) -> FWDIS_W<0> {
        FWDIS_W::new(self)
    }
    ///Bit 8 - Fm+ drive capability on PB6 enable bit
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<8> {
        I2C_PB6_FMP_W::new(self)
    }
    ///Bit 9 - Fm+ drive capability on PB7 enable bit
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<9> {
        I2C_PB7_FMP_W::new(self)
    }
    ///Bit 10 - Fm+ drive capability on PB8 enable bit
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<10> {
        I2C_PB8_FMP_W::new(self)
    }
    ///Bit 11 - Fm+ drive capability on PB9 enable bit
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<11> {
        I2C_PB9_FMP_W::new(self)
    }
    ///Bit 12 - I2C1 Fm+ drive capability enable bit
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<12> {
        I2C1_FMP_W::new(self)
    }
    ///Bit 13 - I2C2 Fm+ drive capability enable bit
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<13> {
        I2C2_FMP_W::new(self)
    }
    ///Bit 14 - I2C3 Fm+ drive capability enable bit
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<14> {
        I2C3_FMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SYSCFG configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
