use fixedbitset::FixedBitSet;
use rand::{thread_rng, Rng};
use std::time::SystemTime;
use uuid::Uuid;


#[derive(Debug)]
pub struct Span {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_id: Option<SpanId>,
    pub options: Option<TraceOptions>,
    pub state: Option<TraceState>,
    start_time: SystemTime,
    end_time: Option<SystemTime>,
}

impl From<SpanContext> for Span {
    fn from(context: SpanContext) -> Span {
        let parent_id = context.span_id;
        context.span_id = SpanId::new();

        Span{
            trace_id: context.trace_id,
            span_id: context.span_id,
            parent_id: Some(parent_id),
            options: context.options,
            state: context.state,
            start_time: SystemTime::now(),
            end_time: None
        }
    }
}

impl Span {
    fn close(&mut self) -> () {
        self.end_time = Some(SystemTime::now());
    }

    fn new(trace: Option<TraceId>, options: Option<TraceOptions>, state: Option<TraceState>) -> Span {
        let trace_id = match trace {
            Some(id) => id,
            None => TraceId(Uuid::new_v4())
        };

        Span{
            trace_id: trace_id,
            span_id: SpanId::new(),
            parent_id: None,
            options: options,
            state: state,
            start_time: SystemTime::now(),
            end_time: None,
        }
    }

    fn child(parent_id: Option<SpanId>, trace_id: TraceId) -> Span {
        Span{
            trace_id: trace_id,
            span_id: SpanId::new(),
            parent_id: parent_id,
            options: None,
            state: None,
            start_time: SystemTime::now(),
            end_time: None,
        }
    }
}

#[derive(Debug)]
struct SpanContext {
    trace_id: TraceId,
    // Not sure that we really want to store this here but for now...
    span_id: SpanId,
    options: Option<TraceOptions>,
    state: Option<TraceState>,
}

impl From<Span> for SpanContext {
    fn from(span: Span) -> SpanContext {
      SpanContext{
          trace_id: span.trace_id,
          span_id: span.span_id,
          options: span.options,
          state: span.state,
      }
    }
}

impl SpanContext {
    fn new(trace: Option<TraceId>) -> SpanContext {
        let trace_id = match trace {
            Some(id) => id,
            None => TraceId(Uuid::new_v4())
        };

        SpanContext{
            trace_id: trace_id,
            span_id: SpanId::new(),
            options: trace_opts,
            state: state,
        }
    }
}

#[derive(Debug)]
struct TraceOptions(FixedBitSet);

impl TraceOptions {
    fn new() -> TraceOptions {
        TraceOptions(FixedBitSet::with_capacity(8))
    }

    fn is_sampling(&self) -> bool {
        self.0.contains(1)
    }
}

#[derive(Debug)]
struct TraceId(Uuid); 

#[derive(Debug)]
struct TraceState {
    pairs: Vec<TracePair>,
}

#[derive(Debug)]
struct TracePair {
    key: String,
    value: String,
}

#[derive(Debug)]
pub struct SpanId([u8; 8]);

impl SpanId {
  fn new() -> SpanId {
      let mut id = [0u8; 8];
      thread_rng().fill(&mut id);

      SpanId(id)
  }
}