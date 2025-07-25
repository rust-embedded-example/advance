#[doc = "Register `TDL1R` reader"]
pub type R = crate::R<Tdl1rSpec>;
#[doc = "Register `TDL1R` writer"]
pub type W = crate::W<Tdl1rSpec>;
#[doc = "Field `DATA0` reader - DATA0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - DATA0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - DATA2"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - DATA3"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<'_, Tdl1rSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, Tdl1rSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, Tdl1rSpec> {
        Data2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<'_, Tdl1rSpec> {
        Data3W::new(self, 24)
    }
}
#[doc = "mailbox data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdl1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdl1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdl1rSpec;
impl crate::RegisterSpec for Tdl1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdl1r::R`](R) reader structure"]
impl crate::Readable for Tdl1rSpec {}
#[doc = "`write(|w| ..)` method takes [`tdl1r::W`](W) writer structure"]
impl crate::Writable for Tdl1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDL1R to value 0"]
impl crate::Resettable for Tdl1rSpec {}
