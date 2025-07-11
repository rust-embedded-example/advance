#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `AFRL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl0R = crate::FieldReader;
#[doc = "Field `AFRL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl1R = crate::FieldReader;
#[doc = "Field `AFRL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl2R = crate::FieldReader;
#[doc = "Field `AFRL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl3R = crate::FieldReader;
#[doc = "Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl4R = crate::FieldReader;
#[doc = "Field `AFRL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl5R = crate::FieldReader;
#[doc = "Field `AFRL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl6R = crate::FieldReader;
#[doc = "Field `AFRL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl7R = crate::FieldReader;
#[doc = "Field `AFRL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Afrl7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&self) -> Afrl0R {
        Afrl0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&self) -> Afrl1R {
        Afrl1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&self) -> Afrl2R {
        Afrl2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&self) -> Afrl3R {
        Afrl3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&self) -> Afrl4R {
        Afrl4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&self) -> Afrl5R {
        Afrl5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&self) -> Afrl6R {
        Afrl6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&self) -> Afrl7R {
        Afrl7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&mut self) -> Afrl0W<'_, AfrlSpec> {
        Afrl0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&mut self) -> Afrl1W<'_, AfrlSpec> {
        Afrl1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&mut self) -> Afrl2W<'_, AfrlSpec> {
        Afrl2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&mut self) -> Afrl3W<'_, AfrlSpec> {
        Afrl3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&mut self) -> Afrl4W<'_, AfrlSpec> {
        Afrl4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&mut self) -> Afrl5W<'_, AfrlSpec> {
        Afrl5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&mut self) -> Afrl6W<'_, AfrlSpec> {
        Afrl6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&mut self) -> Afrl7W<'_, AfrlSpec> {
        Afrl7W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AfrlSpec {}
