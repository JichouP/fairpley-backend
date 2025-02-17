use serde::{Deserialize, Serialize};

// MARK: Reject

/// # Reject
///
/// 回復可能なエラーを表す
///
/// [`RejectKind`] とメッセージのペア
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq, Hash)]
#[error("{kind}: {message}")]
pub struct Reject {
    kind: RejectKind,
    message: String,
}

impl Reject {
    /// 新しい [`Reject`] を作成する。
    pub fn new(kind: RejectKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// [`RejectKind::BadRequest`] の [`Reject`] を作成する。
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(RejectKind::BadRequest, message)
    }

    /// [`RejectKind::NotFound`] の [`Reject`] を作成する。
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(RejectKind::NotFound, message)
    }

    /// [`RejectKind::Unauthorized`] の [`Reject`] を作成する。
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(RejectKind::Unauthorized, message)
    }

    /// [`RejectKind`] の参照を返す
    pub fn as_kind(&self) -> &RejectKind {
        &self.kind
    }

    /// [`RejectKind`] を取り出す
    pub fn into_kind(self) -> RejectKind {
        self.kind
    }

    /// メッセージの参照を返す
    pub fn as_message(&self) -> &str {
        &self.message
    }

    /// メッセージを取り出す
    pub fn into_message(self) -> String {
        self.message
    }
}

// MARK: RejectKind

/// 回復可能なエラーの種類
#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectKind {
    /// バリデーションエラー
    BadRequest,
    /// リソースが見つからなかった
    NotFound,
    /// 認証エラー
    Unauthorized,
}

impl std::fmt::Display for RejectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::Unauthorized => "Unauthorized",
        };
        f.write_str(s)
    }
}

// MARK: Failure

/// # Failure
///
/// 回復可能なエラーと回復不能なエラーを表す。
///
/// [`Failure::Reject`] 回復可能なエラー  
/// [`Failure::Error`] 回復不能なエラー
///
pub enum Failure<R = Reject, E = anyhow::Error> {
    Reject(R),
    Error(E),
}

impl<R, E> ::std::fmt::Display for Failure<R, E>
where
    R: ::std::fmt::Display,
    E: ::std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reject(r) => r.fmt(f),
            Self::Error(e) => e.fmt(f),
        }
    }
}

impl From<Reject> for Failure {
    fn from(r: Reject) -> Self {
        Self::Reject(r)
    }
}

impl From<anyhow::Error> for Failure {
    fn from(e: anyhow::Error) -> Self {
        Self::Error(e)
    }
}

impl Failure {
    pub fn reject_bad_request(message: impl Into<String>) -> Self {
        Self::Reject(Reject::bad_request(message))
    }

    pub fn reject_not_found(message: impl Into<String>) -> Self {
        Self::Reject(Reject::not_found(message))
    }

    pub fn reject_unauthorized(message: impl Into<String>) -> Self {
        Self::Reject(Reject::unauthorized(message))
    }
}
