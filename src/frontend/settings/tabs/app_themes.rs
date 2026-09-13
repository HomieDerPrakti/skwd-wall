use crate::contracts::daemon::AppThemesResult;
use crate::i18n::tr;

use super::{ActionId, Control, Row};

pub(super) fn rows(result: Option<&AppThemesResult>) -> Vec<Row> {
    let mut rows: Vec<_> = result.map_or_else(Vec::new, |result| {
        result
            .apps
            .iter()
            .map(|app| Row {
                title: app.name.clone(),
                desc: tr("settings-app-themes-desc").into(),
                control: Control::AppTheme { app: app.clone() },
            })
            .collect()
    });
    rows.push(Row {
        title: tr("settings-app-themes-scan").into(),
        desc: tr(if result.is_some() {
            "settings-app-themes-scan-desc"
        } else {
            "settings-app-themes-loading"
        })
        .into(),
        control: Control::ActionBtn {
            id: ActionId::RefreshAppThemes,
            label: tr("settings-app-themes-refresh").into(),
        },
    });
    rows
}
