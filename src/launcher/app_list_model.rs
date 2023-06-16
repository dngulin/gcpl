use super::QmlLauncher;

use crate::qml_bridge;
use qmetaobject::prelude::*;
use qmetaobject::QMetaType;
use std::collections::HashMap;
use strum::{EnumIter, FromRepr, IntoStaticStr};

#[derive(Clone, Copy, FromRepr, IntoStaticStr, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[repr(i32)]
enum FieldId {
    Name = qmetaobject::USER_ROLE + 1,
    Icon,
}

impl From<FieldId> for i32 {
    fn from(value: FieldId) -> Self {
        value as i32
    }
}

impl QAbstractListModel for QmlLauncher {
    fn row_count(&self) -> i32 {
        self.launcher.items.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let item_idx = index.row() as usize;
        self.get_item_field(item_idx, role).unwrap_or_default()
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        qml_bridge::role_names::<FieldId>()
    }
}

impl QmlLauncher {
    fn get_item_field(&self, index: usize, role: i32) -> Option<QVariant> {
        let item = self.launcher.items.get(index)?;
        let field_id = FieldId::from_repr(role)?;

        let value = match field_id {
            FieldId::Name => QString::from(item.name.as_str()).to_qvariant(),
            FieldId::Icon => QString::from(item.icon.as_str()).to_qvariant(),
        };

        Some(value)
    }
}
