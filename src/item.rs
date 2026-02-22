#[derive(Debug, PartialEq)]
pub enum RlpItem {
    String(Vec<u8>),
    List(Vec<RlpItem>),
}

impl RlpItem {
    pub fn new_string(bytes: Vec<u8>) -> Self {
        RlpItem::String(bytes)
    }

    pub fn new_list(items: Vec<RlpItem>) -> Self {
        RlpItem::List(items)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, RlpItem::String(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, RlpItem::List(_))
    }

    pub fn as_string(&self) -> Option<&Vec<u8>> {
        match self {
            RlpItem::String(bytes) => Some(bytes),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<RlpItem>> {
        match self {
            RlpItem::List(items) => Some(items),
            _ => None,
        }
    }
}
