use thiserror::Error;

#[derive(Error, Debug)]
pub enum SEESessionError {

}

#[derive(Error, Debug)]
pub enum APIError {
    #[error("内部错误")]
    InternalError,
    #[error("身份验证失败")]
    AuthorizationFailed,
    #[error("Header中未收到Authentication参数，无法进行身份验证")]
    AuthorizationHeaderMissing,
    #[error("Authentication Token非法，请确认Authentication Token正确传递")]
    IllegalAuthenticationToken,
    #[error("Authentication Token已过期，请重新生成/获取")]
    ExpiredAuthenticationToken,
    #[error("通过Authentication Token的验证失败")]
    AuthenticationTokenFailed,
    #[error("账户读写")]
    AccountRWError,
    #[error("您的账户当前处于非活动状态。请检查账户信息")]
    AccountInactive,
    #[error("您的账户不存在")]
    AccountNotExist,
    #[error("您的账户已被锁定，请联系客服解锁")]
    AccountLocked,
    #[error("您的账户已欠费，请充值后重试")]
    AccountArrears,
    #[error("无法成功访问您的账户，请稍后重试")]
    AccountAccessFailed,
    #[error("API 调用错误")]
    APIInvokeError,
    #[error("API 调用参数有误，请检查文档")]
    APIInvokeParamError,
    #[error("模型不存在，请检查模型代码")]
    ModelNotExist,
    #[error("当前模型不支持该调用方式")]
    ModelNotSupportInvokeType,
    #[error("未正常接收到参数")]
    MissingParam,
    #[error("参数非法。请检查文档")]
    IllegalParam,
    #[error("参数冲突，请检查文档")]
    ParamConflict,
    #[error("您无权访问该API")]
    NoPermission,
    #[error("API已下线")]
    APIDeprecated,
    #[error("API不存在")]
    APINotExist,
    #[error("API 调用流程出错")]
    APIInvokeFlowError,
    #[error("您已有请求")]
    RequestExist,
    #[error("获取异步请求结果时，请使用task_id")]
    UseTaskId,
    #[error("任务不存在")]
    TaskNotExist,
    #[error("网络错误，请联系客服")]
    NetworkError,
    #[error("网络错误，请联系客服")]
    NetworkError2,
    #[error("API 运行错误")]
    APIRuntimeError,
    #[error("Prompt 超长")]
    PromptTooLong,
    #[error("API 调用被策略阻止")]
    APIPolicyBlock,
    #[error("系统检测到输入或生成内容可能包含不安全或敏感内容，请您避免输入易产生敏感内容的提示语，感谢您的配合")]
    SensitiveContentDetected,
    #[error("您当前使用该API的并发数过高，请降低并发，或联系客服增加限额")]
    ConcurrentLimit,
    #[error("您当前使用该API的频率过高，请降低频率，或联系客服增加限额")]
    FrequencyLimit,
    #[error("该 API 已达今日调用次数限额，如有更多需求，请联系客服购买")]
    DailyLimit,
    #[error("当前API请求过多，请稍后重试")]
    TooManyRequests,
    #[error("未知错误")]
    UnknownError,
}

impl APIError {
    pub fn code_to_error(code: i32) -> APIError {
        match code {
            500 => APIError::InternalError,
            1000 => APIError::AuthorizationFailed,
            1001 => APIError::AuthorizationHeaderMissing,
            1002 => APIError::IllegalAuthenticationToken,
            1003 => APIError::ExpiredAuthenticationToken,
            1004 => APIError::AuthenticationTokenFailed,
            1100 => APIError::AccountRWError,
            1110 => APIError::AccountInactive,
            1111 => APIError::AccountNotExist,
            1112 => APIError::AccountLocked,
            1113 => APIError::AccountArrears,
            1120 => APIError::AccountAccessFailed,
            1200 => APIError::APIInvokeError,
            1210 => APIError::APIInvokeParamError,
            1211 => APIError::ModelNotExist,
            1212 => APIError::ModelNotSupportInvokeType,
            1213 => APIError::MissingParam,
            1214 => APIError::IllegalParam,
            1215 => APIError::ParamConflict,
            1220 => APIError::NoPermission,
            1221 => APIError::APIDeprecated,
            1222 => APIError::APINotExist,
            1230 => APIError::APIInvokeFlowError,
            1231 => APIError::RequestExist,
            1232 => APIError::UseTaskId,
            1233 => APIError::TaskNotExist,
            1334 => APIError::NetworkError,
            1335 => APIError::NetworkError2,
            1260 => APIError::APIRuntimeError,
            1261 => APIError::PromptTooLong,
            1300 => APIError::APIPolicyBlock,
            1301 => APIError::SensitiveContentDetected,
            1302 => APIError::ConcurrentLimit,
            1303 => APIError::FrequencyLimit,
            1304 => APIError::DailyLimit,
            1305 => APIError::TooManyRequests,
            _ => APIError::UnknownError,
        }
    }

    pub fn error_code(&self) -> i32 {
        match self {
            APIError::InternalError => 500,
            APIError::AuthorizationFailed => 1000,
            APIError::AuthorizationHeaderMissing => 1001,
            APIError::IllegalAuthenticationToken => 1002,
            APIError::ExpiredAuthenticationToken => 1003,
            APIError::AuthenticationTokenFailed => 1004,
            APIError::AccountRWError => 1100,
            APIError::AccountInactive => 1110,
            APIError::AccountNotExist => 1111,
            APIError::AccountLocked => 1112,
            APIError::AccountArrears => 1113,
            APIError::AccountAccessFailed => 1120,
            APIError::APIInvokeError => 1200,
            APIError::APIInvokeParamError => 1210,
            APIError::ModelNotExist => 1211,
            APIError::ModelNotSupportInvokeType => 1212,
            APIError::MissingParam => 1213,
            APIError::IllegalParam => 1214,
            APIError::ParamConflict => 1215,
            APIError::NoPermission => 1220,
            APIError::APIDeprecated => 1221,
            APIError::APINotExist => 1222,
            APIError::APIInvokeFlowError => 1230,
            APIError::RequestExist => 1231,
            APIError::UseTaskId => 1232,
            APIError::TaskNotExist => 1233,
            APIError::NetworkError => 1334,
            APIError::NetworkError2 => 1335,
            APIError::APIRuntimeError => 1260,
            APIError::PromptTooLong => 1261,
            APIError::APIPolicyBlock => 1300,
            APIError::SensitiveContentDetected => 1301,
            APIError::ConcurrentLimit => 1302,
            APIError::FrequencyLimit => 1303,
            APIError::DailyLimit => 1304,
            APIError::TooManyRequests => 1305,
            APIError::UnknownError => -1,
        }
    }

}