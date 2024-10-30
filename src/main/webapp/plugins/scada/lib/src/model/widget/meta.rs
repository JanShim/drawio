use implicit_clone::unsync::IString;
use web_sys::FormData;

use super::NULL_UUID;

#[derive(Debug, PartialEq, Clone)]
pub struct WidgetForm {
    pub uuid: IString,
    pub name: IString,
    pub group: IString,
}

impl WidgetForm {
    pub fn is_new_item(&self) -> bool {
        self.uuid == NULL_UUID
    }
}

impl Default for WidgetForm {
    fn default() -> Self {
        Self { 
            uuid: NULL_UUID.into(), 
            name: Default::default(),
            group: Default::default(),
        }
    }
}

impl From<FormData> for WidgetForm {
    fn from(data: FormData) -> Self {
        Self { 
            uuid: data.get("uuid").as_string().unwrap_or_default().into(), 
            name: data.get("name").as_string().unwrap_or_default().into(), 
            group: data.get("group").as_string().unwrap_or_default().into(), 
        }
    }
}


// ==========================================================
#[cfg(test)]
mod tests {


}