#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ItemKind {
    File,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub location: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TagKind {
    Tag,
    Phone,
    Email,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Tag {
    pub kind: TagKind,
    pub text: String,
}
