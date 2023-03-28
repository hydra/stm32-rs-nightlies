///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SADD` reader - Slave address bit 0
pub type SADD_R = crate::FieldReader<u16, u16>;
///Field `SADD` writer - Slave address bit 0
pub type SADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u16, u16, 10, O>;
///Field `RD_WRN` reader - Transfer direction
pub type RD_WRN_R = crate::BitReader<RD_WRN_A>;
///Transfer direction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_WRN_A {
    ///0: Master requests a write transfer
    Write = 0,
    ///1: Master requests a read transfer
    Read = 1,
}
impl From<RD_WRN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_WRN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RD_WRN_A {
        match self.bits {
            false => RD_WRN_A::Write,
            true => RD_WRN_A::Read,
        }
    }
    ///Checks if the value of the field is `Write`
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == RD_WRN_A::Write
    }
    ///Checks if the value of the field is `Read`
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == RD_WRN_A::Read
    }
}
///Field `RD_WRN` writer - Transfer direction
pub type RD_WRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RD_WRN_A, O>;
impl<'a, const O: u8> RD_WRN_W<'a, O> {
    ///Master requests a write transfer
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(RD_WRN_A::Write)
    }
    ///Master requests a read transfer
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RD_WRN_A::Read)
    }
}
///Field `ADD10` reader - 10-bit addressing mode
pub type ADD10_R = crate::BitReader<ADD10_A>;
///10-bit addressing mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10_A {
    ///0: The master operates in 7-bit addressing mode
    Bit7 = 0,
    ///1: The master operates in 10-bit addressing mode
    Bit10 = 1,
}
impl From<ADD10_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10_A) -> Self {
        variant as u8 != 0
    }
}
impl ADD10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADD10_A {
        match self.bits {
            false => ADD10_A::Bit7,
            true => ADD10_A::Bit10,
        }
    }
    ///Checks if the value of the field is `Bit7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADD10_A::Bit7
    }
    ///Checks if the value of the field is `Bit10`
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ADD10_A::Bit10
    }
}
///Field `ADD10` writer - 10-bit addressing mode
pub type ADD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ADD10_A, O>;
impl<'a, const O: u8> ADD10_W<'a, O> {
    ///The master operates in 7-bit addressing mode
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADD10_A::Bit7)
    }
    ///The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(ADD10_A::Bit10)
    }
}
///Field `HEAD10R` reader - 10-bit address header only read direction
pub type HEAD10R_R = crate::BitReader<HEAD10R_A>;
///10-bit address header only read direction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEAD10R_A {
    ///0: The master sends the complete 10 bit slave address read sequence
    Complete = 0,
    ///1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
    Partial = 1,
}
impl From<HEAD10R_A> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R_A) -> Self {
        variant as u8 != 0
    }
}
impl HEAD10R_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HEAD10R_A {
        match self.bits {
            false => HEAD10R_A::Complete,
            true => HEAD10R_A::Partial,
        }
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == HEAD10R_A::Complete
    }
    ///Checks if the value of the field is `Partial`
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == HEAD10R_A::Partial
    }
}
///Field `HEAD10R` writer - 10-bit address header only read direction
pub type HEAD10R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, HEAD10R_A, O>;
impl<'a, const O: u8> HEAD10R_W<'a, O> {
    ///The master sends the complete 10 bit slave address read sequence
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(HEAD10R_A::Complete)
    }
    ///The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(HEAD10R_A::Partial)
    }
}
///Field `START` reader - Start generation
pub type START_R = crate::BitReader<STARTR_A>;
///Start generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTR_A {
    ///0: No Start generation
    NoStart = 0,
    ///1: Restart/Start generation
    Start = 1,
}
impl From<STARTR_A> for bool {
    #[inline(always)]
    fn from(variant: STARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STARTR_A {
        match self.bits {
            false => STARTR_A::NoStart,
            true => STARTR_A::Start,
        }
    }
    ///Checks if the value of the field is `NoStart`
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == STARTR_A::NoStart
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTR_A::Start
    }
}
///Start generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW_AW {
    ///1: Restart/Start generation
    Start = 1,
}
impl From<STARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: STARTW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `START` writer - Start generation
pub type START_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, STARTW_AW, O>;
impl<'a, const O: u8> START_W<'a, O> {
    ///Restart/Start generation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW_AW::Start)
    }
}
///Field `STOP` reader - Stop generation
pub type STOP_R = crate::BitReader<STOPR_A>;
///Stop generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPR_A {
    ///0: No Stop generation
    NoStop = 0,
    ///1: Stop generation after current byte transfer
    Stop = 1,
}
impl From<STOPR_A> for bool {
    #[inline(always)]
    fn from(variant: STOPR_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPR_A {
        match self.bits {
            false => STOPR_A::NoStop,
            true => STOPR_A::Stop,
        }
    }
    ///Checks if the value of the field is `NoStop`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPR_A::NoStop
    }
    ///Checks if the value of the field is `Stop`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPR_A::Stop
    }
}
///Stop generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPW_AW {
    ///1: Stop generation after current byte transfer
    Stop = 1,
}
impl From<STOPW_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `STOP` writer - Stop generation
pub type STOP_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, STOPW_AW, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    ///Stop generation after current byte transfer
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPW_AW::Stop)
    }
}
///Field `NACK` reader - NACK generation
pub type NACK_R = crate::BitReader<NACKR_A>;
///NACK generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKR_A {
    ///0: an ACK is sent after current received byte
    Ack = 0,
    ///1: a NACK is sent after current received byte
    Nack = 1,
}
impl From<NACKR_A> for bool {
    #[inline(always)]
    fn from(variant: NACKR_A) -> Self {
        variant as u8 != 0
    }
}
impl NACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACKR_A {
        match self.bits {
            false => NACKR_A::Ack,
            true => NACKR_A::Nack,
        }
    }
    ///Checks if the value of the field is `Ack`
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == NACKR_A::Ack
    }
    ///Checks if the value of the field is `Nack`
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKR_A::Nack
    }
}
///NACK generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKW_AW {
    ///1: a NACK is sent after current received byte
    Nack = 1,
}
impl From<NACKW_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `NACK` writer - NACK generation
pub type NACK_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, NACKW_AW, O>;
impl<'a, const O: u8> NACK_W<'a, O> {
    ///a NACK is sent after current received byte
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(NACKW_AW::Nack)
    }
}
///Field `NBYTES` reader - Number of bytes
pub type NBYTES_R = crate::FieldReader<u8, u8>;
///Field `NBYTES` writer - Number of bytes
pub type NBYTES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, u8, 8, O>;
///Field `RELOAD` reader - NBYTES reload mode
pub type RELOAD_R = crate::BitReader<RELOAD_A>;
///NBYTES reload mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_A {
    ///0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
    Completed = 0,
    ///1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
    NotCompleted = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl RELOAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::Completed,
            true => RELOAD_A::NotCompleted,
        }
    }
    ///Checks if the value of the field is `Completed`
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == RELOAD_A::Completed
    }
    ///Checks if the value of the field is `NotCompleted`
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == RELOAD_A::NotCompleted
    }
}
///Field `RELOAD` writer - NBYTES reload mode
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RELOAD_A, O>;
impl<'a, const O: u8> RELOAD_W<'a, O> {
    ///The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(RELOAD_A::Completed)
    }
    ///The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(RELOAD_A::NotCompleted)
    }
}
///Field `AUTOEND` reader - Automatic end mode
pub type AUTOEND_R = crate::BitReader<AUTOEND_A>;
///Automatic end mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOEND_A {
    ///0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
    Software = 0,
    ///1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
    Automatic = 1,
}
impl From<AUTOEND_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTOEND_A {
        match self.bits {
            false => AUTOEND_A::Software,
            true => AUTOEND_A::Automatic,
        }
    }
    ///Checks if the value of the field is `Software`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == AUTOEND_A::Software
    }
    ///Checks if the value of the field is `Automatic`
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AUTOEND_A::Automatic
    }
}
///Field `AUTOEND` writer - Automatic end mode
pub type AUTOEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, AUTOEND_A, O>;
impl<'a, const O: u8> AUTOEND_W<'a, O> {
    ///Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(AUTOEND_A::Software)
    }
    ///Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTOEND_A::Automatic)
    }
}
///Field `PECBYTE` reader - Packet error checking byte
pub type PECBYTE_R = crate::BitReader<PECBYTER_A>;
///Packet error checking byte
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTER_A {
    ///0: No PEC transfer
    NoPec = 0,
    ///1: PEC transmission/reception is requested
    Pec = 1,
}
impl From<PECBYTER_A> for bool {
    #[inline(always)]
    fn from(variant: PECBYTER_A) -> Self {
        variant as u8 != 0
    }
}
impl PECBYTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECBYTER_A {
        match self.bits {
            false => PECBYTER_A::NoPec,
            true => PECBYTER_A::Pec,
        }
    }
    ///Checks if the value of the field is `NoPec`
    #[inline(always)]
    pub fn is_no_pec(&self) -> bool {
        *self == PECBYTER_A::NoPec
    }
    ///Checks if the value of the field is `Pec`
    #[inline(always)]
    pub fn is_pec(&self) -> bool {
        *self == PECBYTER_A::Pec
    }
}
///Packet error checking byte
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTEW_AW {
    ///1: PEC transmission/reception is requested
    Pec = 1,
}
impl From<PECBYTEW_AW> for bool {
    #[inline(always)]
    fn from(variant: PECBYTEW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PECBYTE` writer - Packet error checking byte
pub type PECBYTE_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, PECBYTEW_AW, O>;
impl<'a, const O: u8> PECBYTE_W<'a, O> {
    ///PEC transmission/reception is requested
    #[inline(always)]
    pub fn pec(self) -> &'a mut W {
        self.variant(PECBYTEW_AW::Pec)
    }
}
impl R {
    ///Bits 0:9 - Slave address bit 0
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Transfer direction
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 10-bit addressing mode
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 10-bit address header only read direction
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Stop generation
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NACK generation
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic end mode
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bits 0:9 - Slave address bit 0
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SADD_W<0> {
        SADD_W::new(self)
    }
    ///Bit 10 - Transfer direction
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<10> {
        RD_WRN_W::new(self)
    }
    ///Bit 11 - 10-bit addressing mode
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<11> {
        ADD10_W::new(self)
    }
    ///Bit 12 - 10-bit address header only read direction
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<12> {
        HEAD10R_W::new(self)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<13> {
        START_W::new(self)
    }
    ///Bit 14 - Stop generation
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<14> {
        STOP_W::new(self)
    }
    ///Bit 15 - NACK generation
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<15> {
        NACK_W::new(self)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<16> {
        NBYTES_W::new(self)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<24> {
        RELOAD_W::new(self)
    }
    ///Bit 25 - Automatic end mode
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<25> {
        AUTOEND_W::new(self)
    }
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<26> {
        PECBYTE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0400_e000;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
