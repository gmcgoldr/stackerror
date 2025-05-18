//! Provides the [`ErrorCode`] enum.

use std::io::ErrorKind;

/// Error handling codes.
///
/// Provides runtime information that the caller can use to bypass faulty
/// resources or reformulate an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    RuntimeInvalidValue,
    RuntimeInvalidIndex,
    RuntimeInvalidKey,
    RuntimeNotImplemented,
    // HTTP 4xx
    HttpBadRequest,
    HttpUnauthorized,
    HttpPaymentRequired,
    HttpForbidden,
    HttpNotFound,
    HttpMethodNotAllowed,
    HttpNotAcceptable,
    HttpProxyAuthenticationRequired,
    HttpRequestTimeout,
    HttpConflict,
    HttpGone,
    HttpLengthRequired,
    HttpPreconditionFailed,
    HttpPayloadTooLarge,
    HttpUriTooLong,
    HttpUnsupportedMediaType,
    HttpRangeNotSatisfiable,
    HttpExpectationFailed,
    HttpImATeapot,
    HttpMisdirectedRequest,
    HttpUnprocessableEntity,
    HttpLocked,
    HttpFailedDependency,
    HttpTooEarly,
    HttpUpgradeRequired,
    HttpPreconditionRequired,
    HttpTooManyRequests,
    HttpRequestHeaderFieldsTooLarge,
    HttpUnavailableForLegalReasons,
    // HTTP 5xx
    HttpInternalServerError,
    HttpNotImplemented,
    HttpBadGateway,
    HttpServiceUnavailable,
    HttpGatewayTimeout,
    HttpHttpVersionNotSupported,
    HttpVariantAlsoNegotiates,
    HttpInsufficientStorage,
    HttpLoopDetected,
    HttpNotExtended,
    HttpNetworkAuthenticationRequired,
    // IO
    IoNotFound,
    IoPermissionDenied,
    IoConnectionRefused,
    IoConnectionReset,
    IoConnectionAborted,
    IoNotConnected,
    IoAddrInUse,
    IoAddrNotAvailable,
    IoBrokenPipe,
    IoAlreadyExists,
    IoWouldBlock,
    IoInvalidInput,
    IoInvalidData,
    IoTimedOut,
    IoWriteZero,
    IoInterrupted,
    IoUnsupported,
    IoUnexpectedEof,
    IoOutOfMemory,
    IoOther,
}

impl ErrorCode {
    /// Construct from an HTTP error code value.
    pub fn from_http_value(value: u16) -> Option<Self> {
        Some(match value {
            // 4xx
            400 => Self::HttpBadRequest,
            401 => Self::HttpUnauthorized,
            402 => Self::HttpPaymentRequired,
            403 => Self::HttpForbidden,
            404 => Self::HttpNotFound,
            405 => Self::HttpMethodNotAllowed,
            406 => Self::HttpNotAcceptable,
            407 => Self::HttpProxyAuthenticationRequired,
            408 => Self::HttpRequestTimeout,
            409 => Self::HttpConflict,
            410 => Self::HttpGone,
            411 => Self::HttpLengthRequired,
            412 => Self::HttpPreconditionFailed,
            413 => Self::HttpPayloadTooLarge,
            414 => Self::HttpUriTooLong,
            415 => Self::HttpUnsupportedMediaType,
            416 => Self::HttpRangeNotSatisfiable,
            417 => Self::HttpExpectationFailed,
            418 => Self::HttpImATeapot,
            421 => Self::HttpMisdirectedRequest,
            422 => Self::HttpUnprocessableEntity,
            423 => Self::HttpLocked,
            424 => Self::HttpFailedDependency,
            425 => Self::HttpTooEarly,
            426 => Self::HttpUpgradeRequired,
            428 => Self::HttpPreconditionRequired,
            429 => Self::HttpTooManyRequests,
            431 => Self::HttpRequestHeaderFieldsTooLarge,
            451 => Self::HttpUnavailableForLegalReasons,
            // 5xx
            500 => Self::HttpInternalServerError,
            501 => Self::HttpNotImplemented,
            502 => Self::HttpBadGateway,
            503 => Self::HttpServiceUnavailable,
            504 => Self::HttpGatewayTimeout,
            505 => Self::HttpHttpVersionNotSupported,
            506 => Self::HttpVariantAlsoNegotiates,
            507 => Self::HttpInsufficientStorage,
            508 => Self::HttpLoopDetected,
            510 => Self::HttpNotExtended,
            511 => Self::HttpNetworkAuthenticationRequired,
            _ => return None,
        })
    }

    /// Convert to its corresponding HTTP value, if any.
    pub fn to_http_value(code: ErrorCode) -> Option<u16> {
        Some(match code {
            // 4xx
            ErrorCode::HttpBadRequest => 400,
            ErrorCode::HttpUnauthorized => 401,
            ErrorCode::HttpPaymentRequired => 402,
            ErrorCode::HttpForbidden => 403,
            ErrorCode::HttpNotFound => 404,
            ErrorCode::HttpMethodNotAllowed => 405,
            ErrorCode::HttpNotAcceptable => 406,
            ErrorCode::HttpProxyAuthenticationRequired => 407,
            ErrorCode::HttpRequestTimeout => 408,
            ErrorCode::HttpConflict => 409,
            ErrorCode::HttpGone => 410,
            ErrorCode::HttpLengthRequired => 411,
            ErrorCode::HttpPreconditionFailed => 412,
            ErrorCode::HttpPayloadTooLarge => 413,
            ErrorCode::HttpUriTooLong => 414,
            ErrorCode::HttpUnsupportedMediaType => 415,
            ErrorCode::HttpRangeNotSatisfiable => 416,
            ErrorCode::HttpExpectationFailed => 417,
            ErrorCode::HttpImATeapot => 418,
            ErrorCode::HttpMisdirectedRequest => 421,
            ErrorCode::HttpUnprocessableEntity => 422,
            ErrorCode::HttpLocked => 423,
            ErrorCode::HttpFailedDependency => 424,
            ErrorCode::HttpTooEarly => 425,
            ErrorCode::HttpUpgradeRequired => 426,
            ErrorCode::HttpPreconditionRequired => 428,
            ErrorCode::HttpTooManyRequests => 429,
            ErrorCode::HttpRequestHeaderFieldsTooLarge => 431,
            ErrorCode::HttpUnavailableForLegalReasons => 451,
            // 5xx
            ErrorCode::HttpInternalServerError => 500,
            ErrorCode::HttpNotImplemented => 501,
            ErrorCode::HttpBadGateway => 502,
            ErrorCode::HttpServiceUnavailable => 503,
            ErrorCode::HttpGatewayTimeout => 504,
            ErrorCode::HttpHttpVersionNotSupported => 505,
            ErrorCode::HttpVariantAlsoNegotiates => 506,
            ErrorCode::HttpInsufficientStorage => 507,
            ErrorCode::HttpLoopDetected => 508,
            ErrorCode::HttpNotExtended => 510,
            ErrorCode::HttpNetworkAuthenticationRequired => 511,
            _ => return None,
        })
    }

    /// Construct from an IO error kind.
    pub fn from_io_kind(kind: ErrorKind) -> Option<Self> {
        Some(match kind {
            ErrorKind::NotFound => Self::IoNotFound,
            ErrorKind::PermissionDenied => Self::IoPermissionDenied,
            ErrorKind::ConnectionRefused => Self::IoConnectionRefused,
            ErrorKind::ConnectionReset => Self::IoConnectionReset,
            ErrorKind::ConnectionAborted => Self::IoConnectionAborted,
            ErrorKind::NotConnected => Self::IoNotConnected,
            ErrorKind::AddrInUse => Self::IoAddrInUse,
            ErrorKind::AddrNotAvailable => Self::IoAddrNotAvailable,
            ErrorKind::BrokenPipe => Self::IoBrokenPipe,
            ErrorKind::AlreadyExists => Self::IoAlreadyExists,
            ErrorKind::WouldBlock => Self::IoWouldBlock,
            ErrorKind::InvalidInput => Self::IoInvalidInput,
            ErrorKind::InvalidData => Self::IoInvalidData,
            ErrorKind::TimedOut => Self::IoTimedOut,
            ErrorKind::WriteZero => Self::IoWriteZero,
            ErrorKind::Interrupted => Self::IoInterrupted,
            ErrorKind::Unsupported => Self::IoUnsupported,
            ErrorKind::UnexpectedEof => Self::IoUnexpectedEof,
            ErrorKind::OutOfMemory => Self::IoOutOfMemory,
            ErrorKind::Other => Self::IoOther,
            _ => return None,
        })
    }

    /// Convert to its corresponding `std::io::ErrorKind`, if any.
    pub fn to_io_kind(self) -> Option<ErrorKind> {
        let kind = match self {
            Self::IoNotFound => ErrorKind::NotFound,
            Self::IoPermissionDenied => ErrorKind::PermissionDenied,
            Self::IoConnectionRefused => ErrorKind::ConnectionRefused,
            Self::IoConnectionReset => ErrorKind::ConnectionReset,
            Self::IoConnectionAborted => ErrorKind::ConnectionAborted,
            Self::IoNotConnected => ErrorKind::NotConnected,
            Self::IoAddrInUse => ErrorKind::AddrInUse,
            Self::IoAddrNotAvailable => ErrorKind::AddrNotAvailable,
            Self::IoBrokenPipe => ErrorKind::BrokenPipe,
            Self::IoAlreadyExists => ErrorKind::AlreadyExists,
            Self::IoWouldBlock => ErrorKind::WouldBlock,
            Self::IoInvalidInput => ErrorKind::InvalidInput,
            Self::IoInvalidData => ErrorKind::InvalidData,
            Self::IoTimedOut => ErrorKind::TimedOut,
            Self::IoWriteZero => ErrorKind::WriteZero,
            Self::IoInterrupted => ErrorKind::Interrupted,
            Self::IoUnsupported => ErrorKind::Unsupported,
            Self::IoUnexpectedEof => ErrorKind::UnexpectedEof,
            Self::IoOutOfMemory => ErrorKind::OutOfMemory,
            Self::IoOther => ErrorKind::Other,
            _ => return None,
        };
        Some(kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    /// A few well-chosen HTTP codes should round-trip.
    #[test]
    fn http_roundtrip() {
        let samples = [
            (404, ErrorCode::HttpNotFound),
            (418, ErrorCode::HttpImATeapot),
            (500, ErrorCode::HttpInternalServerError),
        ];

        for (code, variant) in samples {
            // forward
            assert_eq!(ErrorCode::from_http_value(code), Some(variant));
            // backward
            assert_eq!(ErrorCode::to_http_value(variant), Some(code));
        }
    }

    /// Unknown HTTP codes – or non-HTTP variants – must fail gracefully.
    #[test]
    fn http_unknown() {
        // a status that is not in the table
        assert_eq!(ErrorCode::from_http_value(299), None);

        // a non-HTTP variant cannot be rendered as an HTTP status
        assert_eq!(
            ErrorCode::to_http_value(ErrorCode::RuntimeInvalidValue),
            None
        );
    }

    /// Typical IO kinds should also round-trip.
    #[test]
    fn io_roundtrip() {
        let samples = [
            (ErrorKind::NotFound, ErrorCode::IoNotFound),
            (ErrorKind::PermissionDenied, ErrorCode::IoPermissionDenied),
            (ErrorKind::UnexpectedEof, ErrorCode::IoUnexpectedEof),
        ];

        for (kind, variant) in samples {
            // forward
            assert_eq!(ErrorCode::from_io_kind(kind), Some(variant));
            // backward
            assert_eq!(variant.to_io_kind(), Some(kind));
        }
    }
}
