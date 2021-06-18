// Copyright 2021 Black Tea Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hyper::http::response::Builder as HyperHttpResponseBuilder;
use hyper::{header, Body, Response, StatusCode};
use serde::Serialize;

macro_rules! status_response {
    ($func: ident, $status: expr) => {
        #[allow(non_snake_case, missing_docs)]
        pub fn $func() -> HttpResponseBuilder {
            HttpResponseBuilder::new(HyperHttpResponseBuilder::new().status($status))
        }
    };
}

pub struct HttpResponse<B = Body> {
    res: Response<B>,
}

impl HttpResponse<Body> {
    #[inline]
    pub fn res(self) -> Response<Body> {
        self.res
    }
}

pub struct HttpResponseBuilder {
    builder: HyperHttpResponseBuilder,
}

impl HttpResponseBuilder {
    pub fn new(builder: HyperHttpResponseBuilder) -> Self {
        Self { builder }
    }

    pub fn text(self, value: String) -> Response<Body> {
        let builder = self.builder;
        let contains = if let Some(header) = builder.headers_ref() {
            header.contains_key(header::CONTENT_TYPE)
        } else {
            true
        };

        if !contains {
            let builder = builder.header("Content-Type", mime::TEXT_PLAIN.to_string());
            return builder.body(Body::from(value)).unwrap();
        }

        builder.body(Body::from(value)).unwrap()
    }

    pub fn json(self, value: impl Serialize) -> Response<Body> {
        let builder = self.builder;
        match serde_json::to_string(&value) {
            Ok(body) => {
                let contains = if let Some(header) = builder.headers_ref() {
                    header.contains_key(header::CONTENT_TYPE)
                } else {
                    true
                };

                if !contains {
                    let builder =
                        builder.header("Content-Type", mime::APPLICATION_JSON.to_string());
                    return builder.body(Body::from(body)).unwrap();
                }

                builder.body(Body::from(body)).unwrap()
            }
            Err(_) => HttpResponse::InternalServerError()
                .builder
                .body(Body::from("Error"))
                .unwrap(),
        }
    }
}

impl HttpResponse {
    status_response!(Continue, StatusCode::CONTINUE);
    status_response!(SwitchingProtocols, StatusCode::SWITCHING_PROTOCOLS);
    status_response!(Processing, StatusCode::PROCESSING);

    status_response!(Ok, StatusCode::OK);
    status_response!(Created, StatusCode::CREATED);
    status_response!(Accepted, StatusCode::ACCEPTED);
    status_response!(
        NonAuthoritativeInformation,
        StatusCode::NON_AUTHORITATIVE_INFORMATION
    );

    status_response!(NoContent, StatusCode::NO_CONTENT);
    status_response!(ResetContent, StatusCode::RESET_CONTENT);
    status_response!(PartialContent, StatusCode::PARTIAL_CONTENT);
    status_response!(MultiStatus, StatusCode::MULTI_STATUS);
    status_response!(AlreadyReported, StatusCode::ALREADY_REPORTED);

    status_response!(MultipleChoices, StatusCode::MULTIPLE_CHOICES);
    status_response!(MovedPermanently, StatusCode::MOVED_PERMANENTLY);
    status_response!(Found, StatusCode::FOUND);
    status_response!(SeeOther, StatusCode::SEE_OTHER);
    status_response!(NotModified, StatusCode::NOT_MODIFIED);
    status_response!(UseProxy, StatusCode::USE_PROXY);
    status_response!(TemporaryRedirect, StatusCode::TEMPORARY_REDIRECT);
    status_response!(PermanentRedirect, StatusCode::PERMANENT_REDIRECT);

    status_response!(BadRequest, StatusCode::BAD_REQUEST);
    status_response!(NotFound, StatusCode::NOT_FOUND);
    status_response!(Unauthorized, StatusCode::UNAUTHORIZED);
    status_response!(PaymentRequired, StatusCode::PAYMENT_REQUIRED);
    status_response!(Forbidden, StatusCode::FORBIDDEN);
    status_response!(MethodNotAllowed, StatusCode::METHOD_NOT_ALLOWED);
    status_response!(NotAcceptable, StatusCode::NOT_ACCEPTABLE);
    status_response!(
        ProxyAuthenticationRequired,
        StatusCode::PROXY_AUTHENTICATION_REQUIRED
    );
    status_response!(RequestTimeout, StatusCode::REQUEST_TIMEOUT);
    status_response!(Conflict, StatusCode::CONFLICT);
    status_response!(Gone, StatusCode::GONE);
    status_response!(LengthRequired, StatusCode::LENGTH_REQUIRED);
    status_response!(PreconditionFailed, StatusCode::PRECONDITION_FAILED);
    status_response!(PreconditionRequired, StatusCode::PRECONDITION_REQUIRED);
    status_response!(PayloadTooLarge, StatusCode::PAYLOAD_TOO_LARGE);
    status_response!(UriTooLong, StatusCode::URI_TOO_LONG);
    status_response!(UnsupportedMediaType, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    status_response!(RangeNotSatisfiable, StatusCode::RANGE_NOT_SATISFIABLE);
    status_response!(ExpectationFailed, StatusCode::EXPECTATION_FAILED);
    status_response!(UnprocessableEntity, StatusCode::UNPROCESSABLE_ENTITY);
    status_response!(TooManyRequests, StatusCode::TOO_MANY_REQUESTS);
    status_response!(
        RequestHeaderFieldsTooLarge,
        StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
    );
    status_response!(
        UnavailableForLegalReasons,
        StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
    );

    status_response!(InternalServerError, StatusCode::INTERNAL_SERVER_ERROR);
    status_response!(NotImplemented, StatusCode::NOT_IMPLEMENTED);
    status_response!(BadGateway, StatusCode::BAD_GATEWAY);
    status_response!(ServiceUnavailable, StatusCode::SERVICE_UNAVAILABLE);
    status_response!(GatewayTimeout, StatusCode::GATEWAY_TIMEOUT);
    status_response!(VersionNotSupported, StatusCode::HTTP_VERSION_NOT_SUPPORTED);
    status_response!(VariantAlsoNegotiates, StatusCode::VARIANT_ALSO_NEGOTIATES);
    status_response!(InsufficientStorage, StatusCode::INSUFFICIENT_STORAGE);
    status_response!(LoopDetected, StatusCode::LOOP_DETECTED);

    status_response!(NotExtended, StatusCode::NOT_EXTENDED);
    status_response!(
        NetworkAuthenticationRequired,
        StatusCode::NETWORK_AUTHENTICATION_REQUIRED
    );
}
