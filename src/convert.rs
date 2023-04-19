use crate::{
    error::Result,
    wasi_http::{self, IncomingStream},
};

impl TryFrom<wasi_http::Method> for http::Method {
    type Error = http::Error;

    fn try_from(value: wasi_http::Method) -> Result<Self, Self::Error> {
        use bindings::types::Method;
        Ok(match value.0 {
            Method::Get => Self::GET,
            Method::Head => Self::HEAD,
            Method::Post => Self::POST,
            Method::Put => Self::PUT,
            Method::Delete => Self::DELETE,
            Method::Connect => Self::CONNECT,
            Method::Options => Self::OPTIONS,
            Method::Trace => Self::TRACE,
            Method::Patch => Self::PATCH,
            Method::Other(other) => other.parse()?,
        })
    }
}

impl TryFrom<wasi_http::Scheme> for http::uri::Scheme {
    type Error = http::Error;

    fn try_from(value: wasi_http::Scheme) -> Result<Self, Self::Error> {
        use bindings::types::Scheme;
        Ok(match value.0 {
            Scheme::Http => Self::HTTP,
            Scheme::Https => Self::HTTPS,
            Scheme::Other(other) => other.parse()?,
        })
    }
}

impl TryFrom<&wasi_http::IncomingRequest> for http::Uri {
    type Error = http::Error;

    fn try_from(value: &wasi_http::IncomingRequest) -> Result<Self, Self::Error> {
        let mut builder = http::Uri::builder();
        if let Some(scheme) = value.scheme() {
            builder = builder.scheme(scheme);
        }
        if let Some(authority) = value.authority() {
            builder = builder.authority(authority);
        }
        if let Some(path_with_query) = value.path_with_query() {
            builder = builder.path_and_query(path_with_query);
        }
        builder.build()
    }
}

impl TryFrom<wasi_http::Fields> for http::HeaderMap {
    type Error = http::Error;

    fn try_from(value: wasi_http::Fields) -> Result<Self, Self::Error> {
        value
            .entries()
            .into_iter()
            .map(|(k, v)| {
                Ok((
                    http::HeaderName::try_from(k)?,
                    http::HeaderValue::try_from(v)?,
                ))
            })
            .collect()
    }
}

impl TryFrom<wasi_http::IncomingRequest> for http::Request<IncomingBody> {
    type Error = crate::error::Error;

    fn try_from(value: wasi_http::IncomingRequest) -> Result<Self, Self::Error> {
        let mut builder = http::Request::builder().method(value.method()).uri(&value);
        if let Some(headers) = builder.headers_mut() {
            *headers = value.headers().try_into()?;
        }
        let stream = value.consume()?;
        Ok(builder.body(IncomingBody(stream))?)
    }
}

pub struct IncomingBody(IncomingStream);

impl IncomingBody {
    pub fn finish_trailers(self) -> Result<Option<http::HeaderMap>> {
        Ok(self
            .0
            .finish()
            .map(|fields| fields.try_into())
            .transpose()?)
    }
}

impl std::io::Read for IncomingBody {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
