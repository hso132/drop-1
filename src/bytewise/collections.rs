// Dependencies

use crate::data::Varint;
use std::cmp::Eq;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::hash::Hash;
use super::load::Load;
use super::readable::Readable;
use super::reader::Reader;
use super::size::Size;
use super::writable::Writable;
use super::writer::Writer;

// Implementations

impl<Key: Readable, Value: Readable> Readable for BTreeMap<Key, Value> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Reader>(&self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        visitor.visit(&Varint(self.len() as u32))?;

        for (key, value) in self {
            visitor.visit(key)?;
            visitor.visit(value)?;
        }

        Ok(())
    }
}

impl<Key: Load + Ord, Value: Load> Writable for BTreeMap<Key, Value> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Writer>(&mut self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        let size = Varint::load(visitor)?.0 as usize;
        self.clear();

        for _ in 0..size {
            self.insert(Key::load(visitor)?, Value::load(visitor)?);
        }

        Ok(())
    }
}

impl<Key: Load + Ord, Value: Load> Load for BTreeMap<Key, Value> {
    fn load<From: Writer>(from: &mut From) -> Result<Self, From::Error> {
        let mut map = BTreeMap::<Key, Value>::new();
        from.visit(&mut map)?;
        Ok(map)
    }
}

impl<Key: Readable, Value: Readable> Readable for HashMap<Key, Value> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Reader>(&self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        visitor.visit(&Varint(self.len() as u32))?;

        for (key, value) in self {
            visitor.visit(key)?;
            visitor.visit(value)?;
        }

        Ok(())
    }
}

impl<Key: Load + Eq + Hash, Value: Load> Writable for HashMap<Key, Value> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Writer>(&mut self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        let size = Varint::load(visitor)?.0 as usize;

        self.clear();
        self.reserve(size);

        for _ in 0..size {
            self.insert(Key::load(visitor)?, Value::load(visitor)?);
        }

        Ok(())
    }
}

impl<Key: Load + Eq + Hash, Value: Load> Load for HashMap<Key, Value> {
    fn load<From: Writer>(from: &mut From) -> Result<Self, From::Error> {
        let mut map = HashMap::<Key, Value>::new();
        from.visit(&mut map)?;
        Ok(map)
    }
}

impl<Item: Readable> Readable for LinkedList<Item> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Reader>(&self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        visitor.visit(&Varint(self.len() as u32))?;

        for item in self {
            visitor.visit(item)?;
        }

        Ok(())
    }
}

impl<Item: Load> Writable for LinkedList<Item> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Writer>(&mut self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        let size = Varint::load(visitor)?.0 as usize;
        self.clear();

        for _ in 0..size {
            self.push_back(Item::load(visitor)?);
        }

        Ok(())
    }
}

impl<Item: Load> Load for LinkedList<Item> {
    fn load<From: Writer>(from: &mut From) -> Result<Self, From::Error> {
        let mut list = LinkedList::<Item>::new();
        from.visit(&mut list)?;
        Ok(list)
    }
}

impl<Item: Readable> Readable for VecDeque<Item> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Reader>(&self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        visitor.visit(&Varint(self.len() as u32))?;

        for item in self {
            visitor.visit(item)?;
        }

        Ok(())
    }
}

impl<Item: Load> Writable for VecDeque<Item> {
    const SIZE: Size = Size::variable();

    fn accept<Visitor: Writer>(&mut self, visitor: &mut Visitor) -> Result<(), Visitor::Error> {
        let size = Varint::load(visitor)?.0 as usize;

        self.clear();
        self.reserve(size);

        for _ in 0..size {
            self.push_back(Item::load(visitor)?);
        }

        Ok(())
    }
}

impl<Item: Load> Load for VecDeque<Item> {
    fn load<From: Writer>(from: &mut From) -> Result<Self, From::Error> {
        let mut deque = VecDeque::<Item>::new();
        from.visit(&mut deque)?;
        Ok(deque)
    }
}