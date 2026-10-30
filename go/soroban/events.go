package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// Events provides publishing of contract events.
type Events struct{}

// Event is implemented by types declared with the soroban:contractevent
// directive, for which soroban-go generates the implementation.
type Event interface {
	Topics() Vec[Val]
	Data() Val
}

// PublishEvent publishes the event. It corresponds to Event::publish in the
// Rust SDK.
func (Events) PublishEvent(e Event) {
	host.ContractEvent(e.Topics().handle(), uint64(e.Data().ToVal()))
}

// Publish publishes an event with the topics and data. Prefer declaring event
// types with the soroban:contractevent directive, which include the event in
// the contract spec.
func (Events) Publish(topics Vec[Val], data any) {
	host.ContractEvent(topics.handle(), uint64(ToVal(data)))
}

// Publish publishes the event.
func Publish(e Event) { Events{}.PublishEvent(e) }
