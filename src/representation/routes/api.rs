use axum::{
  body::Body,
  extract::State,
  http::{
    HeaderMap, Response, StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
  },
  response::IntoResponse,
};
use chrono::DateTime;

use crate::{
  config::args::build::{COMMIT_DATE, COMMIT_HASH, COMMIT_TIMESTAMP},
  error::Error,
  representation::{
    app::AppState,
    dto::{DebugPipeName, requests, responses},
    middlewares::{jsessionid::JSessionId, war_path::WarPath, xml::Xml},
  },
};

/// # Errors
#[allow(clippy::too_many_lines)]
pub async fn api(
  State(state): State<AppState>,
  WarPath(war_name): WarPath,
  JSessionId(session_id): JSessionId,
  Xml(request): Xml<requests::Request>,
) -> Result<Response<Body>, Error> {
  let data = match request.body {
    requests::RequestKind::SessionInit(_) => {
      let debug_pipe_name = state.session_service.init(&session_id.clone().into()).await?;
      responses::Response {
        body: responses::ResponseKind::Session(responses::Session {
          id: session_id,
          debug_pipe_name: DebugPipeName::new(debug_pipe_name.to_string()),
        }),
      }
    }
    requests::RequestKind::Disconnect(requests::Disconnect { session_id }) => {
      state.session_service.deinit(&session_id.into()).await?;
      responses::Response {
        body: responses::ResponseKind::Done(responses::Done {}),
      }
    }
    requests::RequestKind::SystemSettingsGet(_) => {
      let body = state.settings_service.get_all().await?;
      responses::Response {
        body: responses::ResponseKind::Settings(responses::Settings { body }),
      }
    }
    requests::RequestKind::SystemCoreInfoGet(_) => {
      let dt = DateTime::parse_from_str(COMMIT_DATE, "%Y-%m-%d %H:%M:%S %:z")?;
      let aswar_date = dt.format("%d/%m/%Y %H:%M:%S").to_string();
      responses::Response {
        body: responses::ResponseKind::CoreInfo(responses::CoreInfo {
          auditor: env!("CARGO_PKG_AUTHORS").to_string(),
          owner: env!("CARGO_PKG_AUTHORS").to_string(),
          version: env!("CARGO_PKG_VERSION").to_string(),
          build: COMMIT_TIMESTAMP.to_string(),
          revision: COMMIT_HASH.to_owned(),
          as_version: env!("CARGO_PKG_VERSION").to_string(),
          aswar_date,
        }),
      }
    }
    requests::RequestKind::SystemServerVersionGet(_) => responses::Response {
      body: responses::ResponseKind::ServerInfo(responses::ServerInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
      }),
    },
    requests::RequestKind::ProtocolInfoGet(_) => responses::Response {
      body: responses::ResponseKind::ProtocolInfo(responses::ProtocolInfo {
        version: "9.54".to_string(),
      }),
    },
    requests::RequestKind::AuthenticationURLGet(_) => responses::Response {
      body: responses::ResponseKind::AuthenticationURL(responses::AuthenticationURL {
        url: format!("/{war_name}/authbasic"),
      }),
    },
    requests::RequestKind::UserInfoGet(_) => responses::Response {
      body: responses::ResponseKind::User(responses::User {
        name: "Тест Тест Тестович".to_owned(),
        short_name: "TEST".to_owned(),
        properties: "|ADMIN|CONTEXT|PICKER|PROFILE DEFAULT|SESSION|".to_owned(),
      }),
    },
    requests::RequestKind::SystemNetAddressSet(requests::SystemNetAddressSet {
      session_id: _,
      mac_address: _,
      ip_address: _,
    })
    | requests::RequestKind::NetworkInformationSet(requests::NetworkInformationSet {
      session_id: _,
      client_name: _,
      client_ip: _,
      client_user: _,
      module_name: _,
    }) => responses::Response {
      body: responses::ResponseKind::Done(responses::Done {}),
    },
    requests::RequestKind::NovoAllowedCheck(_) => responses::Response {
      body: responses::ResponseKind::NovoAllowedCheckResult(responses::NovoAllowedCheckResult {
        value: "1".to_owned(),
      }),
    },
    requests::RequestKind::SystemUserPrivilegedGet(_) => responses::Response {
      body: responses::ResponseKind::UserPrivileged(responses::UserPrivileged {
        is_privileged: "true".to_owned(),
      }),
    },
    requests::RequestKind::UserProfilePropertyGet(_) => responses::Response {
      body: responses::ResponseKind::UserProfileProperty(responses::UserProfileProperty { value: String::new() }),
    },
    requests::RequestKind::SystemOptionEnabledCheck(_) => responses::Response {
      body: responses::ResponseKind::OptionInfo(responses::OptionInfo {
        enabled: "true".to_owned(),
      }),
    },
    requests::RequestKind::UserBelongsGroupCheck(_)
    | requests::RequestKind::ClassNeedCollectionIDCheck(requests::ClassNeedCollectionIDCheck {
      session_id: _,
      class_id: _,
    }) => responses::Response {
      body: responses::ResponseKind::CheckResult(responses::CheckResult { value: "0".to_owned() }),
    },
    requests::RequestKind::TypesGet(_) => responses::Response {
      body: responses::ResponseKind::Types(responses::Types {
        body: vec![responses::Class {
          id: "USER".to_string(),
          name: "Пользователи".to_string(),
          base_class_id: "STRUCTURE".to_string(),
          entity_id: "USER".to_string(),
          menu_caption: "По&льзователи".to_string(),
          is_kernel_type: "0".to_string(),
          class_interface: "Z#USER#INTERFACE.CLASS#USER".to_string(),
          is_accessible: "1".to_string(),
          flags: "0100101110100000000000000".to_string(),
        }],
      }),
    },
    requests::RequestKind::GuidesGroupsGet(_) => responses::Response {
      body: responses::ResponseKind::GuidesGroups(responses::GuidesGroups {
        body: vec![responses::GuidesGroup {
          id: String::new(),
          name: String::new(),
        }],
      }),
    },
    requests::RequestKind::GuidesGet(_) => responses::Response {
      body: responses::ResponseKind::Guides(responses::Guides { body: vec![] }),
    },
    requests::RequestKind::UserMenuGet(_) => responses::Response {
      body: responses::ResponseKind::UserMenu(responses::UserMenu {}),
    },
    requests::RequestKind::ClassViewsGet(requests::ClassViewsGet {
      session_id: _,
      class_id: _,
    }) => responses::Response {
      body: responses::ResponseKind::Views(responses::Views {
        body: vec![responses::View {
          id: "4384".to_owned(),
          name: "Полный список".to_owned(),
          short_name: "VW_CRIT_USER".to_owned(),
          is_default: "1".to_owned(),
          cell_style_script: None,
          properties: "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|".to_owned(),
          distance: "0".to_owned(),
          filter_method_short_name: None,
          filter_method_properties: None,
          object_rights: "0".to_owned(),
          to_printer: "1".to_owned(),
          to_file: "1".to_owned(),
        }],
      }),
    },
    requests::RequestKind::ClassChildrenGet(_) => responses::Response {
      body: responses::ResponseKind::ChildClasses(responses::ChildClasses {}),
    },
    requests::RequestKind::ClassMethodsGroupsUserGet(_) => responses::Response {
      body: responses::ResponseKind::MethodsGroups(responses::MethodsGroups {}),
    },
    requests::RequestKind::ClassMethodsGet(requests::ClassMethodsGet {
      session_id: _,
      class_id: _,
    }) => responses::Response {
      body: responses::ResponseKind::Methods(responses::Methods { body: vec![] }),
    },
    requests::RequestKind::ViewColumnsGet(requests::ViewColumnsGet {
      session_id: _,
      view_id: _,
    }) => responses::Response {
      body: responses::ResponseKind::Columns(responses::Columns {
        body: vec![
          responses::Column {
            name: "Фамилия Имя Отчество".to_string(),
            width: "21".to_string(),
            align: "0".to_string(),
            position: "1".to_string(),
            qual: "NAME".to_string(),
            alias: "C_FIO".to_string(),
            base: "STRING".to_string(),
            is_editable: Some("0".to_string()),
            is_sizeable: "1".to_string(),
            is_cell_style: "0".to_string(),
            is_invisible: "0".to_string(),
            target_class_id: None,
            reference_type: None,
            logging: Some("0".to_string()),
            ability_perform_operation: Some("true".to_string()),
            reference_id: None,
          },
          responses::Column {
            name: "Сетевое имя".to_string(),
            width: "9".to_string(),
            align: "0".to_string(),
            position: "2".to_string(),
            qual: "USERNAME".to_string(),
            alias: "C_USERNAME".to_string(),
            base: "STRING".to_string(),
            is_editable: Some("0".to_string()),
            is_sizeable: "1".to_string(),
            is_cell_style: "0".to_string(),
            is_invisible: "0".to_string(),
            target_class_id: None,
            reference_type: None,
            logging: Some("0".to_string()),
            ability_perform_operation: Some("true".to_string()),
            reference_id: None,
          },
          responses::Column {
            name: "Дата поступления".to_string(),
            width: "21".to_string(),
            align: "2".to_string(),
            position: "3".to_string(),
            qual: "BEG_DATE".to_string(),
            alias: "C_BEG_DATE".to_string(),
            base: "DATE".to_string(),
            is_editable: Some("0".to_string()),
            is_sizeable: "1".to_string(),
            is_cell_style: "0".to_string(),
            is_invisible: "2".to_string(),
            target_class_id: None,
            reference_type: None,
            logging: Some("0".to_string()),
            ability_perform_operation: Some("true".to_string()),
            reference_id: None,
          },
          responses::Column {
            name: "Список признаков должностных лиц".to_string(),
            width: "15".to_string(),
            align: "0".to_string(),
            position: "8".to_string(),
            qual: "MARK_PERS".to_string(),
            alias: "C_MARK_PERS_LIST".to_string(),
            base: "STRING".to_string(),
            is_editable: Some("0".to_string()),
            is_sizeable: "1".to_string(),
            is_cell_style: "0".to_string(),
            is_invisible: "0".to_string(),
            target_class_id: None,
            reference_type: None,
            logging: Some("0".to_string()),
            ability_perform_operation: Some("false".to_string()),
            reference_id: None,
          },
          responses::Column {
            name: "Физическое лицо".to_string(),
            width: "21".to_string(),
            align: "0".to_string(),
            position: "11".to_string(),
            qual: "CL_PRIV_REF.NAME".to_string(),
            alias: "C_NAME_1".to_string(),
            base: "STRING".to_string(),
            is_editable: None,
            is_sizeable: "1".to_string(),
            is_cell_style: "0".to_string(),
            is_invisible: "0".to_string(),
            target_class_id: Some("CL_PRIV".to_string()),
            reference_type: Some("0".to_string()),
            logging: None,
            ability_perform_operation: Some("true".to_string()),
            reference_id: Some("a1.C_CL_PRIV_REF".to_string()),
          },
          responses::Column {
            name: "Пачка".to_string(),
            width: "11".to_string(),
            align: "0".to_string(),
            position: "12".to_string(),
            qual: "OTVET".to_string(),
            alias: "C_OTVET".to_string(),
            base: "STRING".to_string(),
            is_editable: Some("0".to_string()),
            is_sizeable: "1".to_string(),
            is_cell_style: "0".to_string(),
            is_invisible: "0".to_string(),
            target_class_id: None,
            reference_type: None,
            logging: Some("0".to_string()),
            ability_perform_operation: Some("true".to_string()),
            reference_id: None,
          },
          responses::Column {
            name: "id".to_string(),
            width: "20".to_string(),
            align: "0".to_string(),
            position: "14".to_string(),
            qual: "ID".to_string(),
            alias: "C_ID".to_string(),
            base: "NUMBER".to_string(),
            is_editable: None,
            is_sizeable: "1".to_string(),
            is_cell_style: "0".to_string(),
            is_invisible: "2".to_string(),
            target_class_id: None,
            reference_type: None,
            logging: None,
            ability_perform_operation: Some("true".to_string()),
            reference_id: None,
          },
        ],
      }),
    },
    requests::RequestKind::ClassStatesGet(requests::ClassStatesGet {
      session_id: _,
      class_id: _,
    }) => responses::Response {
      body: responses::ResponseKind::States(responses::States {}),
    },
    requests::RequestKind::ClassTransitionsGet(requests::ClassTransitionsGet {
      session_id: _,
      class_id: _,
    }) => responses::Response {
      body: responses::ResponseKind::Transitions(responses::Transitions {}),
    },
  };

  let body = requests::XML_HEADER.to_owned() + &quick_xml::se::to_string(&data)?;

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}
