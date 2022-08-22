use cosmwasm_std::{Attribute, Binary, CosmosMsg, Event, SubMsg};
use serde::Serialize;
use serde_json::Value::Null;
use serde_json::{Map, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Aggregator {
    resp: cosmwasm_std::Response<Binary>,
    data: Map<String, Value>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fold_response(&mut self, module: String, resp: Response) {
        self.data.insert(module, resp.data);
        self.resp
            .events
            .extend_from_slice(resp.response.events.as_slice());
        self.resp
            .attributes
            .extend_from_slice(resp.response.attributes.as_slice());
        self.resp
            .messages
            .extend_from_slice(resp.response.messages.as_slice());
    }

    pub fn aggregate(&mut self) -> cosmwasm_std::Response<Binary> {
        if !self.data.is_empty() {
            let bytes = serde_json::to_vec(&self.data).unwrap().into();
            self.resp.data = Some(bytes);
        }
        self.resp.clone()
    }
}

impl Default for Aggregator {
    fn default() -> Self {
        Aggregator {
            resp: cosmwasm_std::Response::new(),
            data: Map::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub response: cosmwasm_std::Response<Binary>,
    pub data: Value,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            response: cosmwasm_std::Response::new(),
            data: Null,
        }
    }
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an attribute included in the main `wasm` event.
    ///
    /// For working with optional values or optional attributes, see [`add_attributes`][Self::add_attributes].
    pub fn add_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.response = self.response.clone().add_attribute(key, value);
        self
    }

    /// This creates a "fire and forget" message, by using `SubMsg::new()` to wrap it,
    /// and adds it to the list of messages to process.
    pub fn add_message(mut self, msg: impl Into<CosmosMsg<Binary>>) -> Self {
        self.response = self.response.clone().add_message(msg);
        self
    }

    /// This takes an explicit SubMsg (creates via eg. `reply_on_error`)
    /// and adds it to the list of messages to process.
    pub fn add_submessage(mut self, msg: SubMsg<Binary>) -> Self {
        self.response = self.response.clone().add_submessage(msg);
        self
    }

    /// Adds an extra event to the response, separate from the main `wasm` event
    /// that is always created.
    ///
    /// The `wasm-` prefix will be appended by the runtime to the provided type
    /// of event.
    pub fn add_event(mut self, event: Event) -> Self {
        self.response = self.response.clone().add_event(event);
        self
    }

    /// Bulk add attributes included in the main `wasm` event.
    ///
    /// Anything that can be turned into an iterator and yields something
    /// that can be converted into an `Attribute` is accepted.
    ///
    /// ## Examples
    ///
    /// Adding a list of attributes using the pair notation for key and value:
    ///
    /// ```
    /// use cosmwasm_std::Response;
    ///
    /// let attrs = vec![
    ///     ("action", "reaction"),
    ///     ("answer", "42"),
    ///     ("another", "attribute"),
    /// ];
    /// let res: Response = Response::new().add_attributes(attrs.clone());
    /// assert_eq!(res.attributes, attrs);
    /// ```
    ///
    /// Adding an optional value as an optional attribute by turning it into a list of 0 or 1 elements:
    ///
    /// ```
    /// use cosmwasm_std::{Attribute, Response};
    ///
    /// // Some value
    /// let value: Option<String> = Some("sarah".to_string());
    /// let attribute: Option<Attribute> = value.map(|v| Attribute::new("winner", v));
    /// let res: Response = Response::new().add_attributes(attribute);
    /// assert_eq!(res.attributes, [Attribute {
    ///     key: "winner".to_string(),
    ///     value: "sarah".to_string(),
    /// }]);
    ///
    /// // No value
    /// let value: Option<String> = None;
    /// let attribute: Option<Attribute> = value.map(|v| Attribute::new("winner", v));
    /// let res: Response = Response::new().add_attributes(attribute);
    /// assert_eq!(res.attributes.len(), 0);
    /// ```
    pub fn add_attributes<A: Into<Attribute>>(
        mut self,
        attrs: impl IntoIterator<Item = A>,
    ) -> Self {
        self.response = self.response.clone().add_attributes(attrs);
        self
    }

    /// Bulk add "fire and forget" messages to the list of messages to process.
    ///
    /// ## Examples
    ///
    /// ```
    /// use cosmwasm_std::{CosmosMsg, Response};
    ///
    /// fn make_response_with_msgs(msgs: Vec<CosmosMsg>) -> Response {
    ///     Response::new().add_messages(msgs)
    /// }
    /// ```
    pub fn add_messages<M: Into<CosmosMsg<Binary>>>(
        mut self,
        msgs: impl IntoIterator<Item = M>,
    ) -> Self {
        // self.response = self.response.clone().borrow_mut().add_messages(msgs);
        self.response = self.response.clone().add_messages(msgs);
        self
    }

    /// Bulk add explicit SubMsg structs to the list of messages to process.
    ///
    /// ## Examples
    ///
    /// ```
    /// use cosmwasm_std::{SubMsg, Response};
    ///
    /// fn make_response_with_submsgs(msgs: Vec<SubMsg>) -> Response {
    ///     Response::new().add_submessages(msgs)
    /// }
    /// ```
    pub fn add_submessages(mut self, msgs: impl IntoIterator<Item = SubMsg<Binary>>) -> Self {
        self.response = self.response.clone().add_submessages(msgs);
        self
    }

    /// Bulk add custom events to the response. These are separate from the main
    /// `wasm` event.
    ///
    /// The `wasm-` prefix will be appended by the runtime to the provided types
    /// of events.
    pub fn add_events(mut self, events: impl IntoIterator<Item = Event>) -> Self {
        self.response = self.response.clone().add_events(events);
        self
    }

    /// Set the binary data included in the response.
    pub fn set_data(mut self, data: impl Serialize) -> Self {
        self.data = serde_json::to_value(data).unwrap();
        self
    }
}

impl From<Response> for cosmwasm_std::Response<Binary> {
    fn from(r: Response) -> Self {
        let mut cr = cosmwasm_std::Response::new();
        cr.data = match r.data {
            Null => None,
            data => {
                let bs = serde_json::to_vec(&data).unwrap();
                Some(bs.into())
            }
        };
        cr.messages = r.response.messages;
        cr.attributes = r.response.attributes;
        cr.events = r.response.events;
        cr
    }
}
