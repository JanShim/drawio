use implicit_clone::unsync::IString;
use web_sys::FormData;

use super::NULL_UUID;

#[derive(Debug, PartialEq, Clone)]
pub struct DiagramForm {
    pub uuid: IString,
    pub name: IString,
}

impl DiagramForm {
    pub fn is_new_item(&self) -> bool {
        self.uuid == NULL_UUID
    }
}

impl Default for DiagramForm {
    fn default() -> Self {
        Self { 
            uuid: NULL_UUID.into(), 
            name: Default::default() 
        }
    }
}

impl From<FormData> for DiagramForm {
    fn from(data: FormData) -> Self {
        Self { 
            uuid: data.get("uuid").as_string().unwrap_or_default().into(), 
            name: data.get("name").as_string().unwrap_or_default().into(), 
        }
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
 

}