use crate::dbs::Iterable;
use crate::sql::{Explain, Object, Value};
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct Explanation(Vec<ExplainItem>);

impl Explanation {
	pub(super) fn new(e: Option<&Explain>, iterables: &Vec<Iterable>) -> (bool, Option<Self>) {
		match e {
			None => (true, None),
			Some(e) => {
				let mut exp = Self::default();
				for i in iterables {
					exp.add_iter(i);
				}
				(e.0, Some(exp))
			}
		}
	}

	fn add_iter(&mut self, iter: &Iterable) {
		self.0.push(ExplainItem::new_iter(iter));
	}

	pub(super) fn add_fetch(&mut self, count: usize) {
		self.0.push(ExplainItem::new_fetch(count));
	}

	pub(super) fn output(self, results: &mut Vec<Value>) {
		for e in self.0 {
			results.push(e.into());
		}
	}
}

struct ExplainItem {
	name: Value,
	details: Vec<(&'static str, Value)>,
}

impl ExplainItem {
	fn new_fetch(count: usize) -> Self {
		Self {
			name: "Fetch".into(),
			details: vec![("count", count.into())],
		}
	}

	fn new_iter(iter: &Iterable) -> Self {
		match iter {
			Iterable::Value(v) => Self {
				name: "Iterate Value".into(),
				details: vec![("value", v.to_owned())],
			},
			Iterable::Table(t) => Self {
				name: "Iterate Table".into(),
				details: vec![("table", Value::from(t.0.to_owned()))],
			},
			Iterable::Thing(t) => Self {
				name: "Iterate Thing".into(),
				details: vec![("thing", Value::Thing(t.to_owned()))],
			},
			Iterable::Range(r) => Self {
				name: "Iterate Range".into(),
				details: vec![("table", Value::from(r.tb.to_owned()))],
			},
			Iterable::Edges(e) => Self {
				name: "Iterate Edges".into(),
				details: vec![("from", Value::Thing(e.from.to_owned()))],
			},
			Iterable::Mergeable(t, v) => Self {
				name: "Iterate Mergeable".into(),
				details: vec![("thing", Value::Thing(t.to_owned())), ("value", v.to_owned())],
			},
			Iterable::Relatable(t1, t2, t3) => Self {
				name: "Iterate Relatable".into(),
				details: vec![
					("thing-1", Value::Thing(t1.to_owned())),
					("thing-2", Value::Thing(t2.to_owned())),
					("thing-3", Value::Thing(t3.to_owned())),
				],
			},
			Iterable::Index(t, _, io) => Self {
				name: "Iterate Index".into(),
				details: vec![("table", Value::from(t.0.to_owned())), ("plan", io.explain())],
			},
		}
	}
}

impl From<ExplainItem> for Value {
	fn from(i: ExplainItem) -> Self {
		let explain = Object::from(HashMap::from([
			("operation", i.name),
			("detail", Value::Object(Object::from(HashMap::from_iter(i.details)))),
		]));
		Value::from(explain)
	}
}
