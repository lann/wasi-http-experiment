#![allow(dead_code)] // WIP

use bindings::types;

use crate::{
    error::{Error, Result},
    wasi_streams::{self},
};

#[derive(Debug)]
pub struct Method(pub(crate) types::Method);

#[derive(Debug)]
pub struct Scheme(pub(crate) types::Scheme);

pub struct Fields(types::Fields);

impl Fields {
    pub fn new(entries: &[(&str, &str)]) -> Self {
        Self(types::new_fields(entries))
    }

    pub fn entries(&self) -> Vec<(String, String)> {
        types::fields_entries(self.0)
    }
}

pub struct IncomingStream(pub(crate) wasi_streams::InputStream);

impl IncomingStream {
    pub fn finish(self) -> Option<Fields> {
        types::finish_incoming_stream(self.0 .0).map(Fields)
    }
}

impl std::io::Read for IncomingStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

pub struct OutgoingStream(pub(crate) wasi_streams::OutputStream);

impl OutgoingStream {
    pub fn finish(self, trailers: Option<Fields>) {
        types::finish_outgoing_stream(self.0 .0, trailers.map(|fields| fields.0))
    }
}

impl std::io::Write for OutgoingStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

pub struct IncomingRequest(pub(crate) types::IncomingRequest);

impl IncomingRequest {
    pub fn method(&self) -> Method {
        Method(types::incoming_request_method(self.0))
    }

    pub fn path_with_query(&self) -> Option<String> {
        types::incoming_request_path_with_query(self.0)
    }

    pub fn scheme(&self) -> Option<Scheme> {
        types::incoming_request_scheme(self.0).map(Scheme)
    }

    pub fn authority(&self) -> Option<String> {
        types::incoming_request_authority(self.0)
    }

    pub fn headers(&self) -> Fields {
        Fields(types::incoming_request_headers(self.0))
    }

    pub fn consume(self) -> Result<IncomingStream> {
        let handle = types::incoming_request_consume(self.0)
            .map_err(|()| Error::other("incoming_request_consume failed"))?;
        Ok(IncomingStream(wasi_streams::InputStream(handle)))
    }
}

pub struct OutgoingRequest(types::OutgoingRequest);

impl OutgoingRequest {
    pub fn new(
        method: &Method,
        path_with_query: Option<&str>,
        scheme: Option<&Scheme>,
        authority: Option<&str>,
        headers: Fields,
    ) -> Self {
        Self(types::new_outgoing_request(
            &method.0,
            path_with_query,
            scheme.map(|inner| &inner.0),
            authority,
            headers.0,
        ))
    }

    pub fn into_stream(self) -> Result<OutgoingStream> {
        let handle = types::outgoing_request_write(self.0)
            .map_err(|()| Error::other("outgoing_request_write failed"))?;
        Ok(OutgoingStream(wasi_streams::OutputStream(handle)))
    }
}

pub struct ResponseOutparam(pub(crate) types::ResponseOutparam);

impl ResponseOutparam {
    pub fn new(handle: types::ResponseOutparam) -> Self {
        Self(handle)
    }

    pub fn set(self, response: &OutgoingResponse) -> Result<()> {
        // TODO: understand what set(Error) means
        types::set_response_outparam(self.0, Ok(response.0))
            .map_err(|()| Error::other("outgoing_request_write failed"))
    }
}

pub struct OutgoingResponse(types::OutgoingResponse);

impl OutgoingResponse {
    pub fn new(status_code: types::StatusCode, headers: Fields) -> Self {
        Self(types::new_outgoing_response(status_code, headers.0))
    }

    pub fn into_stream(self) -> Result<OutgoingStream> {
        let handle = types::outgoing_response_write(self.0)
            .map_err(|()| Error::other("outgoing_response_write failed"))?;
        Ok(OutgoingStream(wasi_streams::OutputStream(handle)))
    }
}
