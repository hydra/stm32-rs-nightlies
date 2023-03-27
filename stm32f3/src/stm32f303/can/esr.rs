///Register `ESR` reader
pub struct R(crate::R<ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ESR` writer
pub struct W(crate::W<ESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR_SPEC>;
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
impl From<crate::W<ESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EWGF` reader - EWGF
pub type EWGF_R = crate::BitReader<bool>;
///Field `EPVF` reader - EPVF
pub type EPVF_R = crate::BitReader<bool>;
///Field `BOFF` reader - BOFF
pub type BOFF_R = crate::BitReader<bool>;
///Field `LEC` reader - LEC
pub type LEC_R = crate::FieldReader<u8, LEC_A>;
///LEC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEC_A {
    ///0: No Error
    NoError = 0,
    ///1: Stuff Error
    Stuff = 1,
    ///2: Form Error
    Form = 2,
    ///3: Acknowledgment Error
    Ack = 3,
    ///4: Bit recessive Error
    BitRecessive = 4,
    ///5: Bit dominant Error
    BitDominant = 5,
    ///6: CRC Error
    Crc = 6,
    ///7: Set by software
    Custom = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
impl LEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NoError,
            1 => LEC_A::Stuff,
            2 => LEC_A::Form,
            3 => LEC_A::Ack,
            4 => LEC_A::BitRecessive,
            5 => LEC_A::BitDominant,
            6 => LEC_A::Crc,
            7 => LEC_A::Custom,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LEC_A::NoError
    }
    ///Checks if the value of the field is `Stuff`
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC_A::Stuff
    }
    ///Checks if the value of the field is `Form`
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC_A::Form
    }
    ///Checks if the value of the field is `Ack`
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC_A::Ack
    }
    ///Checks if the value of the field is `BitRecessive`
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == LEC_A::BitRecessive
    }
    ///Checks if the value of the field is `BitDominant`
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == LEC_A::BitDominant
    }
    ///Checks if the value of the field is `Crc`
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC_A::Crc
    }
    ///Checks if the value of the field is `Custom`
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == LEC_A::Custom
    }
}
///Field `LEC` writer - LEC
pub type LEC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ESR_SPEC, u8, LEC_A, 3, O>;
impl<'a, const O: u8> LEC_W<'a, O> {
    ///No Error
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LEC_A::NoError)
    }
    ///Stuff Error
    #[inline(always)]
    pub fn stuff(self) -> &'a mut W {
        self.variant(LEC_A::Stuff)
    }
    ///Form Error
    #[inline(always)]
    pub fn form(self) -> &'a mut W {
        self.variant(LEC_A::Form)
    }
    ///Acknowledgment Error
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LEC_A::Ack)
    }
    ///Bit recessive Error
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut W {
        self.variant(LEC_A::BitRecessive)
    }
    ///Bit dominant Error
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut W {
        self.variant(LEC_A::BitDominant)
    }
    ///CRC Error
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(LEC_A::Crc)
    }
    ///Set by software
    #[inline(always)]
    pub fn custom(self) -> &'a mut W {
        self.variant(LEC_A::Custom)
    }
}
///Field `TEC` reader - TEC
pub type TEC_R = crate::FieldReader<u8, u8>;
///Field `REC` reader - REC
pub type REC_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - EWGF
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPVF
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BOFF
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - LEC
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 16:23 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - REC
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 4:6 - LEC
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<4> {
        LEC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///error status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [esr](index.html) module
pub struct ESR_SPEC;
impl crate::RegisterSpec for ESR_SPEC {
    type Ux = u32;
}
///`read()` method returns [esr::R](R) reader structure
impl crate::Readable for ESR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [esr::W](W) writer structure
impl crate::Writable for ESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ESR to value 0
impl crate::Resettable for ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
