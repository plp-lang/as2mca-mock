pub struct CreateSessionReq {
  pub username: Box<str>,
  pub password: Box<str>,
}

pub struct CreateSessionRes {
  pub session_id: Box<str>,
}

pub struct InitSessionReq {
  pub session_id: Box<str>,
}

pub struct InitSessionRes {
  pub session_id: Box<str>,
  pub debig_pipe_name: Box<str>,
}
