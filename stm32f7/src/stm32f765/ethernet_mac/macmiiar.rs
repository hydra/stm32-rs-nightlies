///Register `MACMIIAR` reader
pub struct R(crate::R<MACMIIAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMIIAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMIIAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMIIAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACMIIAR` writer
pub struct W(crate::W<MACMIIAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMIIAR_SPEC>;
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
impl From<crate::W<MACMIIAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMIIAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MB` reader - MII busy
pub type MB_R = crate::BitReader<MB_A>;
///MII busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB_A {
    ///1: This bit is set to 1 by the application to indicate that a read or write access is in progress
    Busy = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
impl MB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MB_A> {
        match self.bits {
            true => Some(MB_A::Busy),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == MB_A::Busy
    }
}
///Field `MB` writer - MII busy
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMIIAR_SPEC, MB_A, O>;
impl<'a, const O: u8> MB_W<'a, O> {
    ///This bit is set to 1 by the application to indicate that a read or write access is in progress
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(MB_A::Busy)
    }
}
///Field `MW` reader - MII write
pub type MW_R = crate::BitReader<MW_A>;
///MII write
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MW_A {
    ///0: Read operation
    Read = 0,
    ///1: Write operation
    Write = 1,
}
impl From<MW_A> for bool {
    #[inline(always)]
    fn from(variant: MW_A) -> Self {
        variant as u8 != 0
    }
}
impl MW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MW_A {
        match self.bits {
            false => MW_A::Read,
            true => MW_A::Write,
        }
    }
    ///Checks if the value of the field is `Read`
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == MW_A::Read
    }
    ///Checks if the value of the field is `Write`
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == MW_A::Write
    }
}
///Field `MW` writer - MII write
pub type MW_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMIIAR_SPEC, MW_A, O>;
impl<'a, const O: u8> MW_W<'a, O> {
    ///Read operation
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(MW_A::Read)
    }
    ///Write operation
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(MW_A::Write)
    }
}
///Field `CR` reader - Clock range
pub type CR_R = crate::FieldReader<u8, CR_A>;
///Clock range
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CR_A {
    ///0: 60-100MHz HCLK/42
    Cr60100 = 0,
    ///1: 100-150 MHz HCLK/62
    Cr100150 = 1,
    ///2: 20-35MHz HCLK/16
    Cr2035 = 2,
    ///3: 35-60MHz HCLK/16
    Cr3560 = 3,
    ///4: 150-168MHz HCLK/102
    Cr150168 = 4,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CR_A> {
        match self.bits {
            0 => Some(CR_A::Cr60100),
            1 => Some(CR_A::Cr100150),
            2 => Some(CR_A::Cr2035),
            3 => Some(CR_A::Cr3560),
            4 => Some(CR_A::Cr150168),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Cr60100`
    #[inline(always)]
    pub fn is_cr_60_100(&self) -> bool {
        *self == CR_A::Cr60100
    }
    ///Checks if the value of the field is `Cr100150`
    #[inline(always)]
    pub fn is_cr_100_150(&self) -> bool {
        *self == CR_A::Cr100150
    }
    ///Checks if the value of the field is `Cr2035`
    #[inline(always)]
    pub fn is_cr_20_35(&self) -> bool {
        *self == CR_A::Cr2035
    }
    ///Checks if the value of the field is `Cr3560`
    #[inline(always)]
    pub fn is_cr_35_60(&self) -> bool {
        *self == CR_A::Cr3560
    }
    ///Checks if the value of the field is `Cr150168`
    #[inline(always)]
    pub fn is_cr_150_168(&self) -> bool {
        *self == CR_A::Cr150168
    }
}
///Field `CR` writer - Clock range
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIAR_SPEC, u8, CR_A, 3, O>;
impl<'a, const O: u8> CR_W<'a, O> {
    ///60-100MHz HCLK/42
    #[inline(always)]
    pub fn cr_60_100(self) -> &'a mut W {
        self.variant(CR_A::Cr60100)
    }
    ///100-150 MHz HCLK/62
    #[inline(always)]
    pub fn cr_100_150(self) -> &'a mut W {
        self.variant(CR_A::Cr100150)
    }
    ///20-35MHz HCLK/16
    #[inline(always)]
    pub fn cr_20_35(self) -> &'a mut W {
        self.variant(CR_A::Cr2035)
    }
    ///35-60MHz HCLK/16
    #[inline(always)]
    pub fn cr_35_60(self) -> &'a mut W {
        self.variant(CR_A::Cr3560)
    }
    ///150-168MHz HCLK/102
    #[inline(always)]
    pub fn cr_150_168(self) -> &'a mut W {
        self.variant(CR_A::Cr150168)
    }
}
///Field `MR` reader - MII register - select the desired MII register in the PHY device
pub type MR_R = crate::FieldReader<u8, u8>;
///Field `MR` writer - MII register - select the desired MII register in the PHY device
pub type MR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACMIIAR_SPEC, u8, u8, 5, O>;
///Field `PA` reader - PHY address - select which of possible 32 PHYs is being accessed
pub type PA_R = crate::FieldReader<u8, u8>;
///Field `PA` writer - PHY address - select which of possible 32 PHYs is being accessed
pub type PA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACMIIAR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bit 0 - MII busy
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MII write
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - Clock range
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 6:10 - MII register - select the desired MII register in the PHY device
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - MII busy
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<0> {
        MB_W::new(self)
    }
    ///Bit 1 - MII write
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MW_W<1> {
        MW_W::new(self)
    }
    ///Bits 2:4 - Clock range
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<2> {
        CR_W::new(self)
    }
    ///Bits 6:10 - MII register - select the desired MII register in the PHY device
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<6> {
        MR_W::new(self)
    }
    ///Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<11> {
        PA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC MII address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macmiiar](index.html) module
pub struct MACMIIAR_SPEC;
impl crate::RegisterSpec for MACMIIAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macmiiar::R](R) reader structure
impl crate::Readable for MACMIIAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macmiiar::W](W) writer structure
impl crate::Writable for MACMIIAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACMIIAR to value 0
impl crate::Resettable for MACMIIAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
