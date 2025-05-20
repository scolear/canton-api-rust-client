# Event1

## Enum Variants

| Name | Description |
|---- | -----|
| ParticipantAuthorizationAdded | Events in transactions can have two primary shapes:  - ACS delta: events can be CreatedEvent or ArchivedEvent - ledger effects: events can be CreatedEvent or ExercisedEvent  In the update service the events are restricted to the events visible for the parties specified in the transaction filter. Each event message type below contains a &#x60;&#x60;witness_parties&#x60;&#x60; field which indicates the subset of the requested parties that can see the event in question. |
| ParticipantAuthorizationChanged | Events in transactions can have two primary shapes:  - ACS delta: events can be CreatedEvent or ArchivedEvent - ledger effects: events can be CreatedEvent or ExercisedEvent  In the update service the events are restricted to the events visible for the parties specified in the transaction filter. Each event message type below contains a &#x60;&#x60;witness_parties&#x60;&#x60; field which indicates the subset of the requested parties that can see the event in question. |
| ParticipantAuthorizationRevoked | Events in transactions can have two primary shapes:  - ACS delta: events can be CreatedEvent or ArchivedEvent - ledger effects: events can be CreatedEvent or ExercisedEvent  In the update service the events are restricted to the events visible for the parties specified in the transaction filter. Each event message type below contains a &#x60;&#x60;witness_parties&#x60;&#x60; field which indicates the subset of the requested parties that can see the event in question. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


