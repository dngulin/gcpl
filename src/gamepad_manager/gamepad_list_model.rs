use super::QmlGamepadManager;
use crate::qml_bridge;

use gilrs::GamepadId;
use qmetaobject::prelude::*;
use std::collections::HashMap;
use strum::{EnumIter, FromRepr, IntoStaticStr};

#[derive(Copy, Clone, Debug, QEnum)]
#[repr(i32)]
pub enum QmlPowerStatus {
    Wired = 1,
    Discharging = 2,
    Charging = 3,
    Charged = 4,
}

pub struct Item {
    pub id: GamepadId,

    pub name: QString,
    pub status: QmlPowerStatus,
    pub charge: i32,
}

#[derive(Clone, Copy, FromRepr, IntoStaticStr, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[repr(i32)]
enum FieldId {
    Name = qmetaobject::USER_ROLE + 1,
    Status,
    Charge,
}

impl From<FieldId> for i32 {
    fn from(value: FieldId) -> Self {
        value as i32
    }
}

impl QAbstractListModel for QmlGamepadManager {
    fn row_count(&self) -> i32 {
        self.gamepad_list.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let item_idx = index.row() as usize;
        self.get_item_field(item_idx, role).unwrap_or_default()
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        qml_bridge::role_names::<FieldId>()
    }
}

impl QmlGamepadManager {
    fn get_item_field(&self, index: usize, role: i32) -> Option<QVariant> {
        let item = self.gamepad_list.get(index)?;
        let field_id = FieldId::from_repr(role)?;

        let value = match field_id {
            FieldId::Name => QVariant::from(&item.name),
            FieldId::Status => QVariant::from(item.status as i32),
            FieldId::Charge => QVariant::from(item.charge),
        };

        Some(value)
    }
}
