use serde::{Deserialize, Serialize};

pub mod messages;

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
#[derive(Debug)]
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

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        Reject::bad_request("不正な入力"),
        RejectKind::BadRequest,
        "不正な入力"
    )]
    #[case(
        Reject::not_found("ユーザーが見つかりません"),
        RejectKind::NotFound,
        "ユーザーが見つかりません"
    )]
    #[case(
        Reject::unauthorized("無効なトークン"),
        RejectKind::Unauthorized,
        "無効なトークン"
    )]
    fn test_reject_creation(
        #[case] reject: Reject,
        #[case] expected_kind: RejectKind,
        #[case] expected_message: &str,
    ) {
        assert_eq!(reject.as_kind(), &expected_kind);
        assert_eq!(reject.as_message(), expected_message);
    }

    #[rstest]
    #[case(RejectKind::BadRequest, "Bad Request")]
    #[case(RejectKind::NotFound, "Not Found")]
    #[case(RejectKind::Unauthorized, "Unauthorized")]
    fn test_reject_kind_display(#[case] kind: RejectKind, #[case] expected: &str) {
        assert_eq!(kind.to_string(), expected);
    }

    #[rstest]
    #[case(RejectKind::BadRequest, "エラーメッセージ")]
    #[case(RejectKind::NotFound, "見つかりません")]
    #[case(RejectKind::Unauthorized, "認証エラー")]
    fn test_reject_into_parts(#[case] kind: RejectKind, #[case] message: &str) {
        let reject = Reject::new(kind.clone(), message);
        assert_eq!(reject.clone().into_kind(), kind);
        assert_eq!(reject.into_message(), message);
    }

    #[rstest]
    #[case(
        Reject::bad_request("不正なリクエスト"),
        "Bad Request: 不正なリクエスト"
    )]
    #[case(
        Reject::not_found("ユーザーが見つかりません"),
        "Not Found: ユーザーが見つかりません"
    )]
    #[case(Reject::unauthorized("認証が必要です"), "Unauthorized: 認証が必要です")]
    fn test_reject_display_formatting(#[case] reject: Reject, #[case] expected: &str) {
        assert_eq!(reject.to_string(), expected);
    }

    #[rstest]
    #[case(
        Failure::reject_bad_request("不正なリクエスト"),
        RejectKind::BadRequest,
        "不正なリクエスト"
    )]
    #[case(
        Failure::reject_not_found("リソースが見つかりません"),
        RejectKind::NotFound,
        "リソースが見つかりません"
    )]
    #[case(
        Failure::reject_unauthorized("認証が必要です"),
        RejectKind::Unauthorized,
        "認証が必要です"
    )]
    fn test_failure_helper_methods(
        #[case] failure: Failure,
        #[case] expected_kind: RejectKind,
        #[case] expected_message: &str,
    ) {
        match failure {
            Failure::Reject(reject) => {
                assert_eq!(reject.as_kind(), &expected_kind);
                assert_eq!(reject.as_message(), expected_message);
            }
            Failure::Error(_) => panic!("Expected Reject variant"),
        }
    }

    #[rstest]
    #[case(
        Reject::bad_request("エラー").into(),
        "Bad Request: エラー",
        true
    )]
    #[case(
        anyhow::anyhow!("システムエラー").into(),
        "システムエラー",
        false
    )]
    fn test_failure_variants(
        #[case] failure: Failure,
        #[case] expected_message: &str,
        #[case] is_reject: bool,
    ) {
        assert_eq!(failure.to_string(), expected_message);
        match failure {
            Failure::Reject(_) => assert!(is_reject, "Expected Reject variant"),
            Failure::Error(_) => assert!(!is_reject, "Expected Error variant"),
        }
    }
}
